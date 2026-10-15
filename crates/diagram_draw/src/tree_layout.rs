//! Layout of a containment tree, for the decomposition a general view shows.
//!
//! A decomposition is a tree of boxes of very different fan-out. Putting each depth in one row
//! makes it as wide as its largest generation, which a page cannot hold. This layout keeps a tree
//! compact in both directions:
//!
//! - children that have no children of their own are listed below their parent, indented, like
//!   the entries of a folder;
//! - other children stand side by side below their parent, and wrap into further rows when a row
//!   would be wider than the wrap width.
//!
//! Everything here is geometry: which element contains which is given, and so is every size.

use std::collections::{HashMap, HashSet};

/// Horizontal step from a parent's left edge to its listed children, and the vertical gap
/// between two listed children.
const LIST_INDENT: f64 = 22.0;
const LIST_GAP: f64 = 10.0;
/// Where the line that listed children hang from runs, from the parent's left edge.
const LIST_SPINE: f64 = 10.0;
/// Extra room below the parent of a list in several columns, for the line to the later columns.
const LIST_BUS: f64 = 8.0;
/// Gaps between subtrees that stand side by side, and between a parent and each row of them.
const SIBLING_GAP: f64 = 24.0;
const ROW_GAP: f64 = 36.0;
/// Room at the left of wrapped rows for the line that reaches the second and later rows.
const TRUNK_GUTTER: f64 = 16.0;

pub(crate) type Point = (f64, f64);

/// One row of subtrees standing side by side: each with its id and its x in the row.
type Row = Vec<(String, Subtree, f64)>;

/// A laid-out tree: the top-left corner of every box, the line from each child's parent to the
/// child (parent end first), and the extent of the whole.
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct TreeLayout {
    pub positions: HashMap<String, Point>,
    pub routes: HashMap<String, Vec<Point>>,
    pub size: (f64, f64),
}

struct Tree<'a> {
    sizes: &'a HashMap<String, (f64, f64)>,
    children: HashMap<&'a str, Vec<&'a str>>,
    wrap: f64,
    list_height: f64,
}

/// A laid-out subtree in its own coordinates, with the x of the centre of its top box.
struct Subtree {
    layout: TreeLayout,
    anchor: f64,
}

impl Subtree {
    fn moved(mut self, dx: f64, dy: f64, into: &mut TreeLayout) -> f64 {
        for (id, (x, y)) in self.layout.positions.drain() {
            into.positions.insert(id, (x + dx, y + dy));
        }
        for (id, route) in self.layout.routes.drain() {
            into.routes.insert(
                id,
                route.into_iter().map(|(x, y)| (x + dx, y + dy)).collect(),
            );
        }
        self.anchor + dx
    }
}

impl Tree<'_> {
    fn size(&self, id: &str) -> (f64, f64) {
        self.sizes.get(id).copied().unwrap_or((0.0, 0.0))
    }

    fn subtree(&self, id: &str) -> Subtree {
        let (width, height) = self.size(id);
        let children = self.children.get(id).map(Vec::as_slice).unwrap_or(&[]);
        let mut layout = TreeLayout::default();
        if children.is_empty() {
            layout.positions.insert(id.to_owned(), (0.0, 0.0));
            layout.size = (width, height);
            return Subtree {
                layout,
                anchor: width / 2.0,
            };
        }
        if children
            .iter()
            .all(|child| !self.children.contains_key(child))
        {
            // A list: the parent, then its children one below the other. A list taller than
            // the list height continues in further columns, so that one wide level does not
            // become one column longer than a page.
            layout.positions.insert(id.to_owned(), (0.0, 0.0));
            let list_height: f64 = children
                .iter()
                .map(|child| self.size(child).1 + LIST_GAP)
                .sum::<f64>()
                - LIST_GAP;
            let columns =
                ((list_height / self.list_height).ceil() as usize).clamp(1, children.len());
            let per_column = children.len().div_ceil(columns);
            let top = if columns > 1 {
                height + LIST_GAP + LIST_BUS
            } else {
                height + LIST_GAP
            };
            let bus = height + LIST_GAP;
            let mut right = width;
            let mut bottom = top;
            let mut column_x = LIST_INDENT;
            for (column, members) in children.chunks(per_column).enumerate() {
                let spine = column_x - LIST_INDENT + LIST_SPINE;
                let mut y = top;
                let mut column_width: f64 = 0.0;
                for child in members {
                    let (child_width, child_height) = self.size(child);
                    layout.positions.insert((*child).to_owned(), (column_x, y));
                    let middle = y + child_height / 2.0;
                    let route = if column == 0 {
                        vec![
                            (LIST_SPINE, height),
                            (LIST_SPINE, middle),
                            (column_x, middle),
                        ]
                    } else {
                        // Along the bus below the parent to this column's own spine.
                        vec![
                            (LIST_SPINE, height),
                            (LIST_SPINE, bus),
                            (spine, bus),
                            (spine, middle),
                            (column_x, middle),
                        ]
                    };
                    layout.routes.insert((*child).to_owned(), route);
                    column_width = column_width.max(child_width);
                    y += child_height + LIST_GAP;
                }
                right = right.max(column_x + column_width);
                bottom = bottom.max(y - LIST_GAP);
                column_x += column_width + SIBLING_GAP;
            }
            layout.size = (right, bottom);
            return Subtree {
                layout,
                anchor: width / 2.0,
            };
        }
        // Rows of subtrees below the parent.
        let (rows, rows_width) = self.rows(children.iter().map(|child| self.subtree(child)));
        let wrapped = rows.len() > 1;
        let gutter = if wrapped { TRUNK_GUTTER } else { 0.0 };
        let total_width = (rows_width + gutter).max(width);
        let parent_x = (total_width - width) / 2.0;
        let parent_centre = parent_x + width / 2.0;
        layout.positions.insert(id.to_owned(), (parent_x, 0.0));
        let first_bus = height + ROW_GAP / 2.0;
        let mut y = height + ROW_GAP;
        for (row_index, row) in rows.into_iter().enumerate() {
            let bus = y - ROW_GAP / 2.0;
            let row_height = row
                .iter()
                .map(|(_, subtree, _)| subtree.layout.size.1)
                .fold(0.0, f64::max);
            for (child, subtree, x) in row {
                let anchor = subtree.moved(gutter + x, y, &mut layout);
                let route = if row_index == 0 {
                    vec![
                        (parent_centre, height),
                        (parent_centre, bus),
                        (anchor, bus),
                        (anchor, y),
                    ]
                } else {
                    // Down the gutter at the left, clear of the rows above.
                    let trunk = TRUNK_GUTTER / 2.0;
                    vec![
                        (parent_centre, height),
                        (parent_centre, first_bus),
                        (trunk, first_bus),
                        (trunk, bus),
                        (anchor, bus),
                        (anchor, y),
                    ]
                };
                layout.routes.insert(child, route);
            }
            y += row_height + ROW_GAP;
        }
        layout.size = (total_width, y - ROW_GAP);
        Subtree {
            layout,
            anchor: parent_centre,
        }
    }

    /// Places subtrees side by side, starting a new row when the next one would pass the wrap
    /// width. Returns the rows as (id, subtree, x) and the width of the widest row.
    fn rows(&self, subtrees: impl Iterator<Item = Subtree>) -> (Vec<Row>, f64) {
        let mut rows: Vec<Row> = vec![Vec::new()];
        let mut x = 0.0;
        let mut widest: f64 = 0.0;
        for subtree in subtrees {
            let width = subtree.layout.size.0;
            if x > 0.0 && x + width > self.wrap {
                rows.push(Vec::new());
                x = 0.0;
            }
            // The id of a subtree is the box that has no route to it within the subtree.
            let id = subtree
                .layout
                .positions
                .keys()
                .find(|id| !subtree.layout.routes.contains_key(*id))
                .cloned()
                .unwrap_or_default();
            rows.last_mut()
                .expect("at least one row")
                .push((id, subtree, x));
            widest = widest.max(x + width);
            x += width + SIBLING_GAP;
        }
        (rows, widest)
    }
}

/// Lays out the forest `parent` describes over the boxes `order` lists. A box whose parent is
/// not in `order`, or that sits on a containment cycle, is a root. Rows wrap at `wrap`.
#[cfg(test)]
pub(crate) fn layout_tree(
    order: &[String],
    sizes: &HashMap<String, (f64, f64)>,
    parent: &HashMap<String, String>,
    wrap: f64,
) -> TreeLayout {
    layout_tree_with(order, sizes, parent, wrap, f64::INFINITY)
}

/// As `layout_tree`, with the height at which a list of children continues in a new column.
pub(crate) fn layout_tree_with(
    order: &[String],
    sizes: &HashMap<String, (f64, f64)>,
    parent: &HashMap<String, String>,
    wrap: f64,
    list_height: f64,
) -> TreeLayout {
    let known: HashSet<&str> = order.iter().map(String::as_str).collect();
    // A box belongs under its parent only when following parents from it ends at a root;
    // otherwise it is on a cycle and is drawn as a root itself.
    let reaches_root = |id: &str| {
        let mut current = id;
        for _ in 0..=order.len() {
            match parent.get(current).map(String::as_str) {
                Some(above) if known.contains(above) => current = above,
                _ => return true,
            }
        }
        false
    };
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut roots = Vec::new();
    for id in order {
        match parent.get(id).map(String::as_str) {
            Some(above) if known.contains(above) && reaches_root(id) => {
                children.entry(above).or_default().push(id.as_str());
            }
            _ => roots.push(id.as_str()),
        }
    }
    let tree = Tree {
        sizes,
        children,
        wrap,
        list_height,
    };
    let (rows, width) = tree.rows(roots.iter().map(|root| tree.subtree(root)));
    let mut layout = TreeLayout::default();
    let mut y = 0.0;
    for row in rows {
        let row_height = row
            .iter()
            .map(|(_, subtree, _)| subtree.layout.size.1)
            .fold(0.0, f64::max);
        for (_, subtree, x) in row {
            subtree.moved(x, y, &mut layout);
        }
        y += row_height + ROW_GAP;
    }
    layout.size = (width, (y - ROW_GAP).max(0.0));
    layout
}

/// Lays the tree out at several wrap widths and list heights and keeps the one `score` rates
/// highest; the narrowest of equals with the fewest list columns, so the choice does not depend on anything but the tree.
pub(crate) fn layout_tree_for(
    order: &[String],
    sizes: &HashMap<String, (f64, f64)>,
    parent: &HashMap<String, String>,
    score: impl Fn((f64, f64)) -> f64,
) -> TreeLayout {
    const WRAPS: [f64; 6] = [480.0, 624.0, 926.0, 1300.0, 1900.0, f64::INFINITY];
    // A list stays in one column unless columns rate higher.
    const LIST_HEIGHTS: [f64; 5] = [f64::INFINITY, 1300.0, 926.0, 624.0, 480.0];
    let mut best: Option<(f64, TreeLayout)> = None;
    for wrap in WRAPS {
        for list_height in LIST_HEIGHTS {
            let layout = layout_tree_with(order, sizes, parent, wrap, list_height);
            let rating = score(layout.size);
            if best.as_ref().is_none_or(|(current, _)| rating > *current) {
                best = Some((rating, layout));
            }
        }
    }
    best.map(|(_, layout)| layout).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boxes(ids: &[&str]) -> (Vec<String>, HashMap<String, (f64, f64)>) {
        let order: Vec<String> = ids.iter().map(|id| (*id).to_owned()).collect();
        let sizes = order.iter().map(|id| (id.clone(), (100.0, 40.0))).collect();
        (order, sizes)
    }

    fn parents(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(child, parent)| ((*child).to_owned(), (*parent).to_owned()))
            .collect()
    }

    fn overlaps(layout: &TreeLayout) -> bool {
        let boxes: Vec<&Point> = layout.positions.values().collect();
        boxes.iter().enumerate().any(|(index, a)| {
            boxes[index + 1..].iter().any(|b| {
                a.0 < b.0 + 100.0 && b.0 < a.0 + 100.0 && a.1 < b.1 + 40.0 && b.1 < a.1 + 40.0
            })
        })
    }

    #[test]
    fn leaves_are_listed_below_their_parent() {
        let (order, sizes) = boxes(&["robot", "a", "b", "c"]);
        let layout = layout_tree(
            &order,
            &sizes,
            &parents(&[("a", "robot"), ("b", "robot"), ("c", "robot")]),
            f64::INFINITY,
        );
        assert_eq!(layout.positions["robot"], (0.0, 0.0));
        assert_eq!(layout.positions["a"], (LIST_INDENT, 50.0));
        assert_eq!(layout.positions["c"], (LIST_INDENT, 150.0));
        assert_eq!(layout.size, (LIST_INDENT + 100.0, 190.0));
        // The line leaves the parent's bottom and enters the child's left side at mid height.
        assert_eq!(
            layout.routes["b"],
            vec![
                (LIST_SPINE, 40.0),
                (LIST_SPINE, 120.0),
                (LIST_INDENT, 120.0)
            ]
        );
        assert!(!layout.routes.contains_key("robot"));
    }

    #[test]
    fn subtrees_stand_side_by_side_and_wrap() {
        let (order, sizes) = boxes(&["robot", "m1", "m2", "m3", "p1", "p2", "p3"]);
        let parent = parents(&[
            ("m1", "robot"),
            ("m2", "robot"),
            ("m3", "robot"),
            ("p1", "m1"),
            ("p2", "m2"),
            ("p3", "m3"),
        ]);
        let wide = layout_tree(&order, &sizes, &parent, f64::INFINITY);
        assert_eq!(wide.positions["m1"].1, wide.positions["m3"].1, "one row");
        assert!(!overlaps(&wide));

        let narrow = layout_tree(&order, &sizes, &parent, 300.0);
        assert!(
            narrow.positions["m3"].1 > narrow.positions["m1"].1,
            "wrapped"
        );
        assert!(narrow.size.0 < wide.size.0 && narrow.size.1 > wide.size.1);
        assert!(!overlaps(&narrow));
        // The line to the wrapped row runs down the gutter, left of every box.
        let left = narrow
            .positions
            .values()
            .map(|(x, _)| *x)
            .fold(f64::MAX, f64::min);
        assert!(narrow.routes["m3"]
            .iter()
            .any(|(x, _)| *x < left && *x >= 0.0));
        // Every box and every line stays inside the extent.
        for layout in [&wide, &narrow] {
            assert!(layout.positions.values().all(|(x, y)| *x >= 0.0
                && x + 100.0 <= layout.size.0
                && y + 40.0 <= layout.size.1));
            assert!(layout
                .routes
                .values()
                .flatten()
                .all(
                    |(x, y)| (0.0..=layout.size.0).contains(x) && (0.0..=layout.size.1).contains(y)
                ));
        }
    }

    #[test]
    fn several_roots_and_cycles_are_all_drawn() {
        let (order, sizes) = boxes(&["a", "b", "x", "y", "orphan"]);
        let layout = layout_tree(
            &order,
            &sizes,
            // `x` and `y` contain each other; `orphan` names a parent that is not drawn.
            &parents(&[("b", "a"), ("x", "y"), ("y", "x"), ("orphan", "gone")]),
            f64::INFINITY,
        );
        assert_eq!(layout.positions.len(), 5);
        assert!(!overlaps(&layout));
        assert_eq!(layout.routes.len(), 1, "only `b` hangs from a parent");
    }

    #[test]
    fn the_best_wrap_is_chosen_by_the_score() {
        let ids: Vec<String> = std::iter::once("root".to_owned())
            .chain((0..12).flat_map(|i| [format!("m{i}"), format!("p{i}")]))
            .collect();
        let sizes = ids.iter().map(|id| (id.clone(), (100.0, 40.0))).collect();
        let parent: HashMap<String, String> = (0..12)
            .flat_map(|i| {
                [
                    (format!("m{i}"), "root".to_owned()),
                    (format!("p{i}"), format!("m{i}")),
                ]
            })
            .collect();
        let squarest = |(width, height): (f64, f64)| -(width - height).abs();
        let layout = layout_tree_for(&ids, &sizes, &parent, squarest);
        let unwrapped = layout_tree(&ids, &sizes, &parent, f64::INFINITY);
        assert!(layout.size.0 < unwrapped.size.0);
        assert_eq!(layout, layout_tree_for(&ids, &sizes, &parent, squarest));
    }

    #[test]
    fn a_long_list_continues_in_columns() {
        let order: Vec<String> = std::iter::once("root".to_owned())
            .chain((0..9).map(|index| format!("leaf{index}")))
            .collect();
        let sizes: HashMap<String, (f64, f64)> =
            order.iter().map(|id| (id.clone(), (100.0, 40.0))).collect();
        let parent: HashMap<String, String> = order[1..]
            .iter()
            .map(|id| (id.clone(), "root".to_owned()))
            .collect();

        let one = layout_tree_with(&order, &sizes, &parent, f64::INFINITY, f64::INFINITY);
        // Nine leaves of 40 with gaps of 10 below a root of 40.
        assert_eq!(one.size, (LIST_INDENT + 100.0, 40.0 + 9.0 * 50.0));

        let three = layout_tree_with(&order, &sizes, &parent, f64::INFINITY, 200.0);
        let column = |id: &str| three.positions[id].0;
        assert_eq!(column("leaf0"), column("leaf2"));
        assert!(column("leaf3") > column("leaf2"));
        assert!(column("leaf6") > column("leaf5"));
        assert!(three.size.1 < one.size.1 / 2.0);
        // Every leaf keeps a line from the root, and the later columns start along the bus.
        assert_eq!(three.routes.len(), 9);
        assert_eq!(three.routes["leaf0"].len(), 3);
        assert_eq!(three.routes["leaf4"].len(), 5);
        // No two boxes overlap.
        let boxes: Vec<(f64, f64)> = order[1..].iter().map(|id| three.positions[id]).collect();
        for (index, a) in boxes.iter().enumerate() {
            for b in &boxes[index + 1..] {
                assert!((a.0 - b.0).abs() >= 100.0 || (a.1 - b.1).abs() >= 40.0);
            }
        }
    }
}
