use crate::factory::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Resource)]
pub struct Production {
    pub factories: HashMap<Entity, Factory>,
    pub factory_connections: HashMap<Entity, Vec<Entity>>, // Maps factory Entity -> downstream factory Entities
    pub sources: Vec<Entity>,                              // Entry points of the production chain
    pub sinks: Vec<Entity>, // Exit points that deposit into global inventory
    pub inventory: HashMap<Item, f64>,
}

impl Production {
    /// Creates a new Production system.
    pub fn new(
        factories: HashMap<Entity, Factory>,
        factory_connections: HashMap<Entity, Vec<Entity>>,
        sources: Vec<Entity>,
        sinks: Vec<Entity>,
        inventory: HashMap<Item, f64>,
    ) -> Self {
        Self {
            factories,
            factory_connections,
            sources,
            sinks,
            inventory,
        }
    }

    /// Standard Getters
    pub fn get_factory(&self, entity: Entity) -> Option<&Factory> {
        self.factories.get(&entity)
    }

    pub fn get_factory_mut(&mut self, entity: Entity) -> Option<&mut Factory> {
        self.factories.get_mut(&entity)
    }

    pub fn get_inventory_amount(&self, item: &Item) -> f64 {
        *self.inventory.get(item).unwrap_or(&0.0)
    }

    /// Standard Setters / Modifiers
    pub fn set_inventory_item(&mut self, item: Item, amount: f64) {
        self.inventory.insert(item, amount);
    }

    pub fn add_factory(&mut self, entity: Entity, factory: Factory) {
        self.factories.insert(entity, factory);
    }

    /// Marks an existing factory entity as a source.
    pub fn add_source(&mut self, entity: Entity) {
        if !self.sources.contains(&entity) {
            self.sources.push(entity);
        }
    }

    /// Marks an existing factory entity as a sink.
    pub fn add_sink(&mut self, entity: Entity) {
        if !self.sinks.contains(&entity) {
            self.sinks.push(entity);
        }
    }

    /// Completely removes a factory, cleaning up its outgoing connections,
    /// incoming target references from other factories, and source/sink designations.
    pub fn remove_factory(&mut self, entity: Entity) {
        // 1. Remove from the factories collection
        self.factories.remove(&entity);

        // 2. Remove outgoing connections originating from this factory
        self.factory_connections.remove(&entity);

        // 3. Remove incoming connections targeting this factory across all other factories
        for targets in self.factory_connections.values_mut() {
            targets.retain(|&e| e != entity);
        }

        // 4. Remove from sources list if present
        self.sources.retain(|&e| e != entity);

        // 5. Remove from sinks list if present
        self.sinks.retain(|&e| e != entity);
    }

    /// Removes an entity from the sinks list (the factory remains in the system as a regular factory).
    pub fn remove_sink(&mut self, entity: Entity) {
        self.sinks.retain(|&e| e != entity);
    }

    /// Removes an entity from the sources list (the factory remains in the system as a regular factory).
    pub fn remove_source(&mut self, entity: Entity) {
        self.sources.retain(|&e| e != entity);
    }

    pub fn remove_connection(&mut self, from: Entity, to: Entity) {
        if let Some(targets) = self.factory_connections.get_mut(&from) {
            targets.retain(|&e| e != to);
        }
    }

    pub fn add_connection(&mut self, from: Entity, to: Entity) {
        self.factory_connections.entry(from).or_default().push(to);
    }

    /// Executes one production cycle using topological sorting and Entity keys.
    pub fn production_cycle(&mut self) {
        // 1. Gather all unique entities involved in the production graph
        let mut all_entities = std::collections::HashSet::new();
        for &e in self.factories.keys() {
            all_entities.insert(e);
        }
        for &e in &self.sources {
            all_entities.insert(e);
        }
        for &e in &self.sinks {
            all_entities.insert(e);
        }
        for (&from, targets) in &self.factory_connections {
            all_entities.insert(from);
            for &to in targets {
                all_entities.insert(to);
            }
        }

        // 2. Calculate in-degrees for topological sorting
        let mut in_degrees: HashMap<Entity, usize> = all_entities.iter().map(|&e| (e, 0)).collect();

        for (_from, targets) in &self.factory_connections {
            for &to in targets {
                *in_degrees.entry(to).or_default() += 1;
            }
        }

        let mut incoming_rates: HashMap<Entity, HashMap<Item, f32>> = HashMap::new();
        let mut inventory_deposits: HashMap<Item, f64> = HashMap::new();

        // 3. Initialize queue with nodes having 0 in-degree (sources and unlinked nodes)
        let mut queue: Vec<Entity> = Vec::new();
        for (&entity, &deg) in &in_degrees {
            if deg == 0 {
                queue.push(entity);
            }
        }

        // 4. Process nodes in dependency order
        while let Some(current_id) = queue.pop() {
            let is_source = self.sources.contains(&current_id);
            let is_sink = self.sinks.contains(&current_id);

            if is_sink {
                // Sinks absorb all incoming rates and deposit into inventory
                if let Some(available_inputs) = incoming_rates.get(&current_id) {
                    for (item, rate) in available_inputs {
                        *inventory_deposits.entry(item.clone()).or_default() += *rate as f64;
                    }
                }
                // Sinks have no outgoing propagation
                continue;
            }

            if is_source {
                // Sources have infinite throughput: supply what downstream factories require
                if let Some(downstreams) = self.factory_connections.get(&current_id) {
                    for &downstream_id in downstreams {
                        if let Some(downstream_factory) = self.factories.get(&downstream_id) {
                            let dest_pool = incoming_rates.entry(downstream_id).or_default();
                            for (item, rate) in &downstream_factory.input {
                                *dest_pool.entry(item.clone()).or_default() += *rate;
                            }
                        }

                        if let Some(deg) = in_degrees.get_mut(&downstream_id) {
                            *deg -= 1;
                            if *deg == 0 {
                                queue.push(downstream_id);
                            }
                        }
                    }
                }
                continue;
            }

            // Regular Factory processing
            if let Some(factory) = self.factories.get(&current_id) {
                let available_inputs = incoming_rates.get(&current_id).cloned().unwrap_or_default();

                let scale_factor = if factory.input.is_empty() {
                    1.0
                } else {
                    let mut min_ratio = 1.0;
                    for (req_item, req_rate) in &factory.input {
                        let supplied = available_inputs.get(req_item).copied().unwrap_or(0.0);
                        let ratio = if *req_rate > 0.0 {
                            supplied / *req_rate
                        } else {
                            1.0
                        };
                        if ratio < min_ratio {
                            min_ratio = ratio;
                        }
                    }
                    min_ratio.clamp(0.0, 1.0)
                };

                let actual_output: Vec<(Item, f32)> = factory
                    .output
                    .iter()
                    .map(|(item, rate)| (item.clone(), rate * scale_factor))
                    .collect();

                // Propagate outputs to downstream connections
                if let Some(downstreams) = self.factory_connections.get(&current_id) {
                    for &downstream_id in downstreams {
                        let dest_pool = incoming_rates.entry(downstream_id).or_default();
                        for (item, rate) in &actual_output {
                            *dest_pool.entry(item.clone()).or_default() += *rate;
                        }

                        if let Some(deg) = in_degrees.get_mut(&downstream_id) {
                            *deg -= 1;
                            if *deg == 0 {
                                queue.push(downstream_id);
                            }
                        }
                    }
                }
            }
        }

        // 5. Apply accumulated inventory deposits atomically
        for (item, amount) in inventory_deposits {
            *self.inventory.entry(item).or_default() += amount;
        }
    }
}
