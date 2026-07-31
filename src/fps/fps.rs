use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

pub fn fps_setup(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        })
        .with_child((
            Text::new("FPS: 0"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::WHITE),
            FpsText,
        ));
}

pub fn fps_update(
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<FpsText>>,
) {
    let mut text = text_query.single_mut().unwrap();
    let fps = 1.0 / time.delta_secs();
    *text = Text::new(format!("FPS: {:.0}", fps));
}