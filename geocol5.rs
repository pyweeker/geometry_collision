use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const SLOW: f32 = 0.3;
const NORMAL: f32 = 1.0;
const FAST: f32 = 10.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)

        //.add_system(change_draw_mode_system)
        

        .add_system(change_number_of_sides)


        

        .add_system(rotate_shape_system)
        

        .add_system(move_shape_system)
        

        .add_system(scale_shape_system)
        
        .run();
}



//#[derive(Component)]
//struct RootCompoOrBundle; 


#[derive(Component)]
struct GeoMorph;  


#[derive(Component)]
struct Rot;  


#[derive(Component)]
struct ScalMorph;



//   https://www.reddit.com/r/bevy/comments/s79ubw/need_help_for_improving_filtering_query_and/



#[derive(Component)]
//struct CompoSpeed(x: f32, y:f32);
struct CompoSpeed {x: f32, y:f32}



//__________________________________________________________________________________________________________




fn change_number_of_sides(mut query: Query<&mut Path, With<GeoMorph>>, time: Res<Time>) {
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


//__________________________________________________________________________________________________________     https://bevy-cheatbook.github.io/pitfalls/split-borrows.html




fn move_shape_system(mut query: Query<(&mut Transform, &CompoSpeed)>, time: Res<Time>) {

    let delta = time.delta_seconds();

    for (mut tf, speed) in query.iter_mut() {

        tf.translation.x += speed.x;
        tf.translation.y += speed.y;

    }


    
}



//__________________________________________________________________________________________________________

fn scale_shape_system(mut query: Query<(&mut Transform, &ScalMorph)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    //for mut transform in query.iter_mut() {
    for (mut tf, scm) in query.iter_mut() {
        tf.scale -= 0.05 * delta;
        
    }
}

//__________________________________________________________________________________________________________

//fn rotate_shape_system(mut query: Query<&mut Transform, &Rot>, time: Res<Time>) {         expected `bool`, found `&Rot`    ATTENTION PARENTHESES
fn rotate_shape_system(mut query: Query<(&mut Transform, &Rot)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut tf, rot) in query.iter_mut() {
        tf.rotate(Quat::from_rotation_z(0.2 * delta));
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
        .insert(ScalMorph);
        



    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::RED, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(-30.0, -20.0, 0.0))
        ))
        .insert(CompoSpeed {x: NORMAL, y: SLOW})
        .insert(GeoMorph);


    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLUE),
                outline_mode: StrokeMode::new(Color::GREEN, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(50.0, 70.0, 0.0))
        ))        
        .insert(GeoMorph);


    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::ORANGE),
                outline_mode: StrokeMode::new(Color::PINK, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(250.0, 270.0, 0.0))
        ))        
        
        
        .insert(CompoSpeed {x: -SLOW, y: -SLOW})
        .insert(Rot)          // expected expression error if you put ; to precedant code line     to fix it .insert(CompoSpeed {x: -SLOW, y: -SLOW})  
        .insert(ScalMorph)
        //.insert(GeoMorph) // expected `()`, found `&mut EntityCommands<'_, '_, '_>`    ===> On Last Insert, finish line code with ;
        .insert(GeoMorph);
        

        
        

        



        
    

    
}
