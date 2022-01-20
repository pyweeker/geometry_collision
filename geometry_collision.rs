use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)

        .add_system(change_draw_mode_system)
        

        .add_system(change_number_of_sides)
        

        .add_system(rotate_shape_system)
        

        .add_system(move_shape_system)
        

        .add_system(scale_shape_system)
        
        .run();
}

#[derive(Component)]
struct BadgeA;  


#[derive(Component)]
struct BadgeB; 

#[derive(Component)]
struct BadgeC; 


trait BadgeTrait {}

impl BadgeTrait for BadgeA {}
impl BadgeTrait for BadgeB {}
impl BadgeTrait for BadgeC {}



//   https://www.reddit.com/r/bevy/comments/s79ubw/need_help_for_improving_filtering_query_and/



#[derive(Bundle)]
struct FunnyShape<T: BadgeTrait> {
    badge: T,
    
}










//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
fn move_shape_system(mut query: Query<&mut Transform, With<ExampleShapeA>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.translation.x += 16.0 * delta;
        transform.translation.y += 8.0 * delta;
    }
}



fn move_shape_systemB(mut query: Query<&mut Transform, With<ExampleShapeB>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.translation.x -= 16.0 * delta;
        transform.translation.y -= 8.0 * delta;
    }
}


fn move_shape_systemC(mut query: Query<&mut Transform, With<ExampleShapeC>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.translation.x += 2.0 * delta;
        transform.translation.y -= 1.0 * delta;
    }
}

//________________________________________________
fn scale_shape_system(mut query: Query<&mut Transform, With<ExampleShapeA>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.scale -= 0.05 * delta;
        
    }
}

fn scale_shape_systemB(mut query: Query<&mut Transform, With<ExampleShapeB>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.scale += 0.15 * delta;
        
    }
}

fn scale_shape_systemC(mut query: Query<&mut Transform, With<ExampleShapeC>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.scale += 0.10 * delta;
        
    }
}
//__________________________________________________________________________________________________________

fn rotate_shape_system(mut query: Query<&mut Transform, With<ExampleShapeA>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}

fn rotate_shape_systemB(mut query: Query<&mut Transform, With<ExampleShapeB>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(-0.2 * delta));
    }
}

fn rotate_shape_systemC(mut query: Query<&mut Transform, With<ExampleShapeC>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(0.5 * delta));
    }
}


//__________________________________________________________________________________________________________


fn change_draw_mode_system(mut query: Query<&mut DrawMode, With<ExampleShapeA>>, time: Res<Time>) {
    let hue = (time.seconds_since_startup() * 50.0) % 360.0;
    let outline_width = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;

    for mut draw_mode in query.iter_mut() {
        if let DrawMode::Outlined {
            ref mut fill_mode,
            ref mut outline_mode,
        } = *draw_mode
        {
            fill_mode.color = Color::hsl(hue as f32, 1.0, 0.5);
            outline_mode.options.line_width = outline_width as f32;
        }
    }
}


fn change_draw_mode_systemB(mut query: Query<&mut DrawMode, With<ExampleShapeB>>, time: Res<Time>) {
    let hue = (time.seconds_since_startup() * 15.0) % 180.0;
    let outline_width = 12.0 + time.seconds_since_startup().sin().abs() * 10.0;

    for mut draw_mode in query.iter_mut() {
        if let DrawMode::Outlined {
            ref mut fill_mode,
            ref mut outline_mode,
        } = *draw_mode
        {
            fill_mode.color = Color::hsl(hue as f32, 1.0, 0.5);
            outline_mode.options.line_width = outline_width as f32;
        }
    }
}


fn change_draw_mode_systemC(mut query: Query<&mut DrawMode, With<ExampleShapeC>>, time: Res<Time>) {
    let hue = (time.seconds_since_startup() * 80.0) % 360.0;
    let outline_width = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;

    for mut draw_mode in query.iter_mut() {
        if let DrawMode::Outlined {
            ref mut fill_mode,
            ref mut outline_mode,
        } = *draw_mode
        {
            fill_mode.color = Color::hsl(hue as f32, 1.0, 0.5,);
            outline_mode.options.line_width = outline_width as f32;
        }
    }
}


//__________________________________________________________________________________________________________



fn change_number_of_sides(mut query: Query<&mut Path, With<ExampleShapeA>>, time: Res<Time>) {
    let sides = ((time.seconds_since_startup() - PI * 2.5).sin() * 2.5 + 5.5).round() as usize;

    for mut path in query.iter_mut() {
        let polygon = shapes::RegularPolygon {
            sides,
            //feature: shapes::RegularPolygonFeature::Radius(200.0),
            feature: shapes::RegularPolygonFeature::Radius(200.0),
            ..shapes::RegularPolygon::default()
        };

        *path = ShapePath::build_as(&polygon);
    }
}



fn change_number_of_sidesB(mut query: Query<&mut Path, With<ExampleShapeB>>, time: Res<Time>) {
    let sides = ((time.seconds_since_startup() - PI * 0.005).sin() * 2.5 + 5.5).round() as usize;

    for mut path in query.iter_mut() {
        let polygon = shapes::RegularPolygon {
            sides,
            //feature: shapes::RegularPolygonFeature::Radius(200.0),
            feature: shapes::RegularPolygonFeature::Radius(75.0),
            ..shapes::RegularPolygon::default()
        };

        *path = ShapePath::build_as(&polygon);
    }
}


fn change_number_of_sidesC(mut query: Query<&mut Path, With<ExampleShapeC>>, time: Res<Time>) {
    let sides = ((time.seconds_since_startup() - PI * 1.5).cos() * 2.5 + 5.5).round() as usize;

    for mut path in query.iter_mut() {
        let polygon = shapes::RegularPolygon {
            sides,
            //feature: shapes::RegularPolygonFeature::Radius(200.0),
            feature: shapes::RegularPolygonFeature::Radius(30.0),
            ..shapes::RegularPolygon::default()
        };

        *path = ShapePath::build_as(&polygon);
    }
}


//__________________________________________________________________________________________________________






fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        //feature: shapes::RegularPolygonFeature::Radius(200.0),
        feature: shapes::RegularPolygonFeature::Radius(75.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(-300.0, -200.0, 0.0))
        ))
        .insert(ExampleShapeA);
    

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::RED),
                outline_mode: StrokeMode::new(Color::YELLOW, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(300.0, 200.0, 0.0))
        ))
        .insert(ExampleShapeB);

    
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(Color::BLUE, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(-75.0, 180.0, 0.0))
        ))
        .insert(ExampleShapeC);
}
