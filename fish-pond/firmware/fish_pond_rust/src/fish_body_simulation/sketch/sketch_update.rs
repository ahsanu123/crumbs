use crate::fish_body_simulation::{
    fish::fish::{Bounds, Fish},
    functions::{Random, dist, map},
    leaf::{duckweed::DuckWeed, leaf::Leaf},
    ripple::ripple::Ripple,
};

pub fn is_overlapping(a: Bounds, b: Bounds) -> bool {
    !(a.right <= b.left || b.right <= a.left || a.bottom <= b.top || b.bottom <= a.top)
}

pub fn detect_fish_leaf_collision(fish: &Fish, leaves: &mut [Leaf]) {
    let center = fish.bounds.center();
    for leaf in leaves {
        let position = leaf.position();
        let distance = dist(center.x, center.y, position.x, position.y);
        if distance < fish.width() * 2.0 {
            leaf.apply_oscillation(center.x, center.y, fish.velocity() / distance);
        }
    }
}

pub fn detect_fish_duckweed_collision(fish: &Fish, weeds: &mut [DuckWeed]) {
    let center = fish.bounds.center();
    for weed in weeds {
        let position = weed.position();
        let distance = dist(center.x, center.y, position.x, position.y);
        if distance < fish.width() * 2.0 {
            weed.apply_vector(center.x, center.y, 0.2 * fish.velocity() / distance);
        }
    }
}

pub fn detect_ripple_leaf_collision(ripple: &Ripple, leaves: &mut [Leaf]) {
    for leaf in leaves {
        let position = leaf.position();
        let distance = dist(ripple.x, ripple.y, position.x, position.y);
        for ring in &ripple.ripple_group {
            if distance > ring.current_radius + leaf.radius {
                break;
            }
            if distance < ring.current_radius - leaf.radius {
                continue;
            }
            leaf.apply_oscillation(
                ripple.x,
                ripple.y,
                map(ring.current_intensity, 0.0, 255.0, 0.0, 1.0),
            );
        }
    }
}

pub fn detect_ripple_duckweed_collision(ripple: &Ripple, weeds: &mut [DuckWeed]) {
    for weed in weeds {
        let position = weed.position();
        let distance = dist(ripple.x, ripple.y, position.x, position.y);
        for ring in &ripple.ripple_group {
            if distance > ring.current_radius + weed.radius {
                break;
            }
            if distance < ring.current_radius - weed.radius {
                continue;
            }
            weed.apply_vector(
                ripple.x,
                ripple.y,
                map(ring.current_intensity, 0.0, 255.0, 0.0, 0.1),
            );
        }
    }
}

pub fn update_plants(
    leaves: &mut [Leaf],
    weeds: &mut [DuckWeed],
    width: f32,
    height: f32,
    rng: &mut Random,
) {
    for leaf in leaves {
        leaf.update();
    }
    for weed in weeds {
        weed.update(width, height, rng);
    }
}
