use std::collections::{BTreeSet, VecDeque};

use elk_core::{Point, Rect, Size};

/// View of a graph for connected-component detection.
pub trait ComponentGraphView {
    type Node: Copy + Ord;

    /// All nodes in the graph.
    fn nodes(&self) -> Vec<Self::Node>;
    /// Neighbors of a node (treat as undirected).
    fn neighbors(&self, n: Self::Node) -> Vec<Self::Node>;
    /// Node bounding box for packing.
    fn bounds(&self, n: Self::Node) -> Rect;
    /// Translate a node by delta.
    fn translate(&mut self, n: Self::Node, dx: f32, dy: f32);
}

#[must_use]
pub fn connected_components<V: ComponentGraphView>(view: &V) -> Vec<Vec<V::Node>> {
    let mut seen: BTreeSet<V::Node> = BTreeSet::new();
    let mut out = Vec::new();

    for start in view.nodes() {
        if !seen.insert(start) {
            continue;
        }
        let mut comp = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(start);
        while let Some(n) = q.pop_front() {
            comp.push(n);
            for nb in view.neighbors(n) {
                if seen.insert(nb) {
                    q.push_back(nb);
                }
            }
        }
        out.push(comp);
    }
    out
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RowPackingOptions {
    pub spacing: f32,
    pub padding: f32,
    pub target_aspect_ratio: f32,
}

/// Pack connected components into rows. Returns overall bounds.
pub fn pack_components_in_rows<V: ComponentGraphView>(
    view: &mut V,
    options: RowPackingOptions,
) -> Rect {
    let comps = connected_components(view);
    if comps.len() <= 1 {
        // Expand with padding only.
        let nodes = view.nodes();
        if nodes.is_empty() {
            return Rect::new(Point::default(), Size::new(options.padding * 2.0, options.padding * 2.0));
        }
        let mut min = Point::new(f32::MAX, f32::MAX);
        let mut max = Point::new(f32::MIN, f32::MIN);
        for n in nodes {
            let r = view.bounds(n);
            min.x = min.x.min(r.origin.x);
            min.y = min.y.min(r.origin.y);
            max.x = max.x.max(r.origin.x + r.size.width);
            max.y = max.y.max(r.origin.y + r.size.height);
        }
        return Rect::new(
            Point::new(min.x - options.padding, min.y - options.padding),
            Size::new((max.x - min.x) + options.padding * 2.0, (max.y - min.y) + options.padding * 2.0),
        );
    }

    // Compute component rects.
    #[derive(Clone)]
    struct Comp<N: Copy> {
        nodes: Vec<N>,
        rect: Rect,
        area: f32,
    }
    let mut metas: Vec<Comp<V::Node>> = comps
        .into_iter()
        .map(|nodes| {
            let mut min = Point::new(f32::MAX, f32::MAX);
            let mut max = Point::new(f32::MIN, f32::MIN);
            for n in &nodes {
                let r = view.bounds(*n);
                min.x = min.x.min(r.origin.x);
                min.y = min.y.min(r.origin.y);
                max.x = max.x.max(r.origin.x + r.size.width);
                max.y = max.y.max(r.origin.y + r.size.height);
            }
            let rect = Rect::new(min, Size::new(max.x - min.x, max.y - min.y));
            let area = rect.size.width.max(1.0) * rect.size.height.max(1.0);
            Comp { nodes, rect, area }
        })
        .collect();

    metas.sort_by(|a, b| b.area.total_cmp(&a.area));

    let target = options.target_aspect_ratio.clamp(0.4, 3.0);
    let total_area: f32 = metas.iter().map(|m| m.area).sum();
    let target_row_width = (total_area * target).sqrt().max(1.0);

    let mut cursor_x = 0.0;
    let mut cursor_y = 0.0;
    let mut row_height = 0.0;
    let mut packed_min = Point::new(f32::MAX, f32::MAX);
    let mut packed_max = Point::new(f32::MIN, f32::MIN);

    for comp in metas {
        if cursor_x > 0.0 && cursor_x + comp.rect.size.width > target_row_width {
            cursor_x = 0.0;
            cursor_y += row_height + options.spacing;
            row_height = 0.0;
        }

        let dx = cursor_x - comp.rect.origin.x + options.padding;
        let dy = cursor_y - comp.rect.origin.y + options.padding;
        for n in &comp.nodes {
            view.translate(*n, dx, dy);
        }

        packed_min.x = packed_min.x.min(cursor_x + options.padding);
        packed_min.y = packed_min.y.min(cursor_y + options.padding);
        packed_max.x = packed_max.x.max(cursor_x + comp.rect.size.width + options.padding);
        packed_max.y = packed_max.y.max(cursor_y + comp.rect.size.height + options.padding);

        cursor_x += comp.rect.size.width + options.spacing;
        row_height = row_height.max(comp.rect.size.height);
    }

    Rect::new(
        Point::new(packed_min.x - options.padding, packed_min.y - options.padding),
        Size::new(
            (packed_max.x - packed_min.x) + options.padding * 2.0,
            (packed_max.y - packed_min.y) + options.padding * 2.0,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Default)]
    struct Mini {
        nodes: Vec<usize>,
        edges: BTreeMap<usize, Vec<usize>>,
        rects: BTreeMap<usize, Rect>,
    }
    impl ComponentGraphView for Mini {
        type Node = usize;
        fn nodes(&self) -> Vec<Self::Node> {
            self.nodes.clone()
        }
        fn neighbors(&self, n: Self::Node) -> Vec<Self::Node> {
            self.edges.get(&n).cloned().unwrap_or_default()
        }
        fn bounds(&self, n: Self::Node) -> Rect {
            self.rects[&n]
        }
        fn translate(&mut self, n: Self::Node, dx: f32, dy: f32) {
            let r = self.rects.get_mut(&n).unwrap();
            r.origin.x += dx;
            r.origin.y += dy;
        }
    }

    #[test]
    fn finds_two_components() {
        let mut m = Mini::default();
        m.nodes = vec![0, 1, 2];
        m.edges.insert(0, vec![1]);
        m.edges.insert(1, vec![0]);
        // node 2 isolated
        m.rects.insert(0, Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0)));
        m.rects.insert(1, Rect::new(Point::new(20.0, 0.0), Size::new(10.0, 10.0)));
        m.rects.insert(2, Rect::new(Point::new(0.0, 20.0), Size::new(10.0, 10.0)));
        let comps = connected_components(&m);
        assert_eq!(comps.len(), 2);
    }

    #[test]
    fn packing_translates_components() {
        let mut m = Mini::default();
        m.nodes = vec![0, 1, 2];
        m.edges.insert(0, vec![1]);
        m.edges.insert(1, vec![0]);
        m.rects.insert(0, Rect::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0)));
        m.rects.insert(1, Rect::new(Point::new(20.0, 0.0), Size::new(10.0, 10.0)));
        m.rects.insert(2, Rect::new(Point::new(0.0, 50.0), Size::new(10.0, 10.0)));
        let before = m.rects[&2].origin;
        let _bounds = pack_components_in_rows(
            &mut m,
            RowPackingOptions {
                spacing: 5.0,
                padding: 10.0,
                target_aspect_ratio: 1.0,
            },
        );
        let after = m.rects[&2].origin;
        assert_ne!(before, after);
    }
}

