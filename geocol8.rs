use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const SLOW: f32 = 0.2;
const NORMAL: f32 = 0.5;
const FAST: f32 = 3.0;

const LITTLE: f32 = 35.0;
const MEDIUM: f32 = 50.0;
const BIG: f32 = 125.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)

        .add_system(change_draw_mode_system)
        

        .add_system(change_number_of_sides)


        

        .add_system(rotateRight_shape_system)
        .add_system(rotateLeft_shape_system)
        

        .add_system(move_shape_system)
        

        .add_system(scale_shape_system)
        
        .run();
}



//#[derive(Component)]
//struct RootCompoOrBundle; 


#[derive(Component)]
struct GeoMorph;  


//#[derive(Component)]
//struct Rot;  

#[derive(Component)]
struct RotRight; 

#[derive(Component)]
struct RotLeft; 

#[derive(Component)]
struct ScalMorph;


#[derive(Component)]
struct ColorMorph;

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
            feature: shapes::RegularPolygonFeature::Radius(LITTLE),
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
fn rotateRight_shape_system(mut query: Query<(&mut Transform, &RotRight)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut tf, rot) in query.iter_mut() {
        tf.rotate(Quat::from_rotation_z(0.2 * delta));
    }
}


fn rotateLeft_shape_system(mut query: Query<(&mut Transform, &RotLeft)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut tf, rot) in query.iter_mut() {
        tf.rotate(Quat::from_rotation_z(-0.8 * delta));
    }
}



//__________________________________________________________________________________________________________


//__________________________________________________________________________________________________________


fn change_draw_mode_system(mut query: Query<(&mut DrawMode, &ColorMorph)>, time: Res<Time>) {
    let hue = (time.seconds_since_startup() * 50.0) % 360.0;
    let outline_width = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;

    for (mut draw_mode, colormorph) in query.iter_mut() {
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
//__________________________________________________________________________________________________________

fn setup_system(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        //feature: shapes::RegularPolygonFeature::Radius(200.0),
        feature: shapes::RegularPolygonFeature::Radius(55.0),
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



    let square_shape = shapes::RegularPolygon {
        sides: 4,        
        feature: shapes::RegularPolygonFeature::Radius(25.0),
        ..shapes::RegularPolygon::default()
    };

    let triangle_shape = shapes::RegularPolygon {
        sides: 3,        
        feature: shapes::RegularPolygonFeature::Radius(MEDIUM),
        ..shapes::RegularPolygon::default()
    };

    let octo_shape = shapes::RegularPolygon {
        sides: 8,        
        feature: shapes::RegularPolygonFeature::Radius(LITTLE),
        ..shapes::RegularPolygon::default()
    };
        



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
            Transform::from_translation(Vec3::new(80.0, 70.0, 0.0))
        ))        
        .insert(RotRight);

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::GREEN),
                outline_mode: StrokeMode::new(Color::BLUE, 10.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(-80.0, -70.0, 0.0))
        ))        
        .insert(RotLeft);


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
        //.insert(Rot)          // expected expression error if you put ; to precedant code line     to fix it .insert(CompoSpeed {x: -SLOW, y: -SLOW})  
        
        //.insert(GeoMorph) // expected `()`, found `&mut EntityCommands<'_, '_, '_>`    ===> On Last Insert, finish line code with ;
        //.insert(GeoMorph);

        //.insert(ScalMorph);
        .insert(RotLeft);


    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(Color::GREEN, 1.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(-310.0, 280.0, 0.0))
        ))        
        
        
        
        .insert(ColorMorph);

    //------------------------------------


    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &square_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(400.0, 300.0, 0.0))
        ))

        .insert(CompoSpeed {x: 0.0, y: -0.1});
        
        

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &triangle_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(Color::GREEN, 1.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(400.0, 0.0, 0.0))
        ))        
        
        
        
        .insert(ColorMorph)
        .insert(CompoSpeed {x: -NORMAL, y: 0.0});

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &octo_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(Color::GREEN, 1.0),
            },
            //Transform::default(),
            Transform::from_translation(Vec3::new(400.0, -300.0, 0.0))
        ))

        .insert(CompoSpeed {x: -NORMAL, y: SLOW})        
        .insert(ColorMorph);      
        
        

        
        

        



        
    

    
}
