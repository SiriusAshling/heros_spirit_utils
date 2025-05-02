use std::collections::HashMap;

use geo::{Centroid, Coord, CoordsIter, KNearestConcaveHull, Polygon};
use image::{Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_antialiased_line_segment_mut, draw_antialiased_polygon_mut},
    pixelops::interpolate,
    point::Point,
};
use lyon::{
    algorithms::hatching::{HatchSegment, Hatcher, HatchingOptions, RegularHatchingPattern},
    geom::LineSegment,
    math::{point, Angle},
    path::{Event, LineCap, Path},
    tessellation::{
        geometry_builder::simple_builder, StrokeOptions, StrokeTessellator, VertexBuffers,
    },
};

use super::{logic::Area, Logic};

#[allow(clippy::type_complexity)]
pub struct Visualizer<'logic> {
    area_shapes: HashMap<u8, Vec<(&'logic Area, Polygon<f32>, geo::Point<f32>)>>,
    area_centroids: HashMap<&'logic String, geo::Point<f32>>,
}

impl<'logic> Visualizer<'logic> {
    pub fn new(logic: &'logic Logic) -> Self {
        let mut area_shapes = HashMap::<_, Vec<_>>::new();
        let mut area_centroids = HashMap::new();

        for (name, area) in &logic.areas {
            let Some(first_id) = area.items.keys().next().or_else(|| area.transfers.first()) else {
                eprintln!("nothing in {name}");
                continue;
            };

            let outline = area
                .items
                .keys()
                .chain(&area.transfers)
                .map(|id| Coord {
                    x: (id.x * 16 + 8) as f32,
                    y: (id.y * 16 + 8) as f32,
                })
                .collect::<Vec<_>>()
                .k_nearest_concave_hull(0);

            let centroid = outline.centroid().unwrap();

            area_shapes
                .entry(first_id.map)
                .or_default()
                .push((area, outline, centroid));
            area_centroids.insert(name, centroid);
        }

        Self {
            area_shapes,
            area_centroids,
        }
    }

    pub fn visualize_areas(&self, map: u8, image: &mut RgbaImage) {
        for (_, outline, _) in &self.area_shapes[&map] {
            if outline.coords_count() > 2 {
                visualize_area(outline, image);
            }
        }
    }

    pub fn visualize_connections(&self, map: u8, image: &mut RgbaImage) {
        for (area, _, centroid) in &self.area_shapes[&map] {
            for path in area.paths.keys() {
                if let Some(target) = self.area_centroids.get(path) {
                    visualize_connection(centroid.0, target.0, image);
                }
            }
        }
    }
}

fn visualize_area(outline: &Polygon<f32>, image: &mut RgbaImage) {
    let mut coords = outline.coords_iter();
    let first = coords.next().unwrap();
    let mut builder = Path::builder();
    builder.begin(point(first.x, first.y));
    for coord in coords {
        builder.line_to(point(coord.x, coord.y));
    }
    builder.close();
    let path = builder.build();

    let mut hatches = Path::builder();
    let mut hatcher = Hatcher::new();
    hatcher.hatch_path(
        path.iter(),
        &HatchingOptions::default()
            .with_angle(Angle::degrees(30.))
            .with_tangents(false)
            .with_tolerance(0.),
        &mut RegularHatchingPattern {
            interval: 3.,
            callback: &mut |segment: &HatchSegment| {
                hatches.add_line_segment(&LineSegment {
                    from: segment.a.position,
                    to: segment.b.position,
                });
            },
        },
    );
    let path = hatches.build();

    let mut current = vec![];
    for event in &path {
        match event {
            Event::Begin { at: point } | Event::Line { to: point, .. } => {
                current.push((point.x.round() as i32, point.y.round() as i32));
            }
            Event::End { .. } => {
                assert!(current.len() == 2);

                draw_antialiased_line_segment_mut(
                    image,
                    current[0],
                    current[1],
                    Rgba::from([0, 0, 255, 255]),
                    interpolate,
                );

                current.clear();
            }
            _ => unreachable!(),
        }
    }
}

fn visualize_connection(start: Coord<f32>, end: Coord<f32>, image: &mut RgbaImage) {
    let mut builder = Path::builder();
    builder.begin(point(start.x, start.y));
    builder.line_to(point(end.x, end.y));
    builder.close();
    let path = builder.build();

    let mut buffers = VertexBuffers::new();
    let mut builder = simple_builder(&mut buffers);
    StrokeTessellator::new()
        .tessellate(
            &path,
            &StrokeOptions::default()
                .with_line_width(3.)
                .with_line_cap(LineCap::Round),
            &mut builder,
        )
        .unwrap();

    let points = buffers
        .vertices
        .into_iter()
        .map(|point| Point::new(point.x.round() as i32, point.y.round() as i32))
        .collect::<Vec<_>>();

    draw_antialiased_polygon_mut(image, &points, Rgba::from([255, 0, 0, 255]), interpolate);
}
