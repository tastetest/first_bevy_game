use bevy::prelude::*;
use bevy_ninepatch::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>)  {

    let panel_texture_handle = asset_server.load("/glassPanel_corners.png");

    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    let content_entity = commands.spawn_bundle(TextBundle {..Default::default()}).id();

    commands.spawn_bundle(
        NinePatchBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(500.), Val::Px(300.)),
                ..Default::default()
            },
            nine_patch_data: NinePatchData::with_single_content(
                                 panel_texture_handle,
                                 nine_patch_handle,
                                 content_entity,
                             ),
                    ..Default::default()

        },
    );
}
    

