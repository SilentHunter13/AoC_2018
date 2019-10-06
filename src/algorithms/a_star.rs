use std::collections::HashMap;

pub trait Cost<P> {
    fn cost(&self, position: &P) -> u32;
}

pub trait Neighbour<P> {
    fn neighbours_iter(&self, pos: P) -> Vec<P>;
}

#[derive(Clone, Copy, Debug)]
struct Attributes<P> {
    g: u32,                 //minimale Kosten, um zu dieser Positio zu kommen
    predecessor: Option<P>, //Vorgünger Knoten
}

pub struct AStar<G, P> {
    open_list: HashMap<P, (u32, Attributes<P>)>, // Prioritätenwarteschlange
    closed_list: HashMap<P, Attributes<P>>,
    map: G,
    next: Option<P>,
}

impl<G, P> AStar<G, P>
where
    P: std::cmp::Eq + std::hash::Hash + Copy + Cost<P>,
    G: Neighbour<P>,
{
    pub fn new(map: G) -> AStar<G, P>
    where
        P: std::cmp::Eq + std::hash::Hash,
    {
        AStar {
            open_list: HashMap::new(),
            closed_list: HashMap::new(),
            map,
            next: None,
        }
    }
    pub fn calc_path(&mut self, start: P, target: P) -> Option<u32> {
        // Initialisierung der Open List, die Closed List ist noch leer
        // (die Priorität bzw. der f-Wert des Startknotens ist unerheblich)
        self.open_list.insert(
            start,
            (
                0,
                Attributes {
                    g: 0,
                    predecessor: None,
                },
            ),
        );
        // diese Schleife wird durchlaufen bis entweder
        // - die optimale Lösung gefunden wurde oder
        // - feststeht, dass keine Lösung existiert
        loop {
            // Knoten mit dem geringsten f-Wert aus der Open List entfernen
            if let Some((current_node, _)) = self.open_list.iter().min_by_key(|x| ((*x).1).0) {
                let copied_current_node = *current_node;

                let (removed_current_node, (_, attr)) = self
                    .open_list
                    .remove_entry(&copied_current_node)
                    .expect("node must be in");

                // Der aktuelle Knoten soll durch nachfolgende Funktionen
                // nicht weiter untersucht werden, damit keine Zyklen entstehen
                self.closed_list.insert(removed_current_node, attr);

                // Wurde das Ziel gefunden?
                if removed_current_node == target {
                    self.next = Some(removed_current_node);
                    break Some(attr.g); //PathFound;
                }

                // Wenn das Ziel noch nicht gefunden wurde: Nachfolgeknoten
                // des aktuellen Knotens auf die Open List setzen
                self.expand_node(removed_current_node, target, attr);
            } else {
                // die Open List ist leer, es existiert kein Pfad zum Ziel
                break None;
            }
        }
    }
    fn expand_node(&mut self, node: P, target: P, attr: Attributes<P>) {
        for successor in self.map.neighbours_iter(node) {
            // wenn der Nachfolgeknoten bereits auf der Closed List ist – tue nichts
            if !self.closed_list.contains_key(&successor) {
                // g-Wert für den neuen Weg berechnen: g-Wert des Vorgängers plus
                // die Kosten der gerade benutzten Kante
                let tentative_g = attr.g + successor.cost(&node);
                let f = tentative_g + successor.cost(&target); //Berechnung der Kosten und Abschätzun zum Ziel hier gleich

                //wenn der Nachfolgeknoten schon in der open_list ist
                if let Some((f_s, attributes)) = self.open_list.get_mut(&successor) {
                    //wenn der neue Weg besser als der alte ist
                    if attributes.g > tentative_g {
                        attributes.predecessor = Some(node);
                        attributes.g = tentative_g;
                        *f_s = f;
                    }
                } else {
                    //Nachfolgeknoten ist noch nicht in der open_list
                    self.open_list.insert(
                        successor,
                        (
                            f,
                            Attributes {
                                g: tentative_g,
                                predecessor: Some(node),
                            },
                        ),
                    );
                }
            }
        }
    }
}

impl<G, P> Iterator for AStar<G, P>
where
    P: std::cmp::Eq + std::hash::Hash + Copy,
{
    type Item = P;

    fn next(&mut self) -> Option<P> {
        let ret_val;
        match self.next {
            None => None,
            Some(next) => {
                ret_val = self.next;
                let next_item = self.closed_list.get(&next).expect("Item not available");
                self.next = next_item.predecessor;
                ret_val
            }
        }
    }
}
