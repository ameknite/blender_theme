// SPDX-License-Identifier: GPL-3.0-or-later

pub mod types;

use core::fmt;
use std::{fs, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use xmlem::{
    display::{Config, EntityMode},
    Document,
};

use crate::types::{BackGroundType, Boolean, Factor, HexColor};

#[derive(Debug, PartialEq)]
pub struct B3dTheme {
    pub bpy: Bpy,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Bpy {
    pub theme: Theme,
    pub theme_style: ThemeStyle,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Theme {
    pub user_interface: UserInterface,
    pub view_3d: View3d,
    pub graph_editor: GraphEditor,
    pub file_browser: FileBrowser,
    pub nla_editor: NlaEditor,
    pub dopesheet_editor: DopesheetEditor,
    pub image_editor: ImageEditor,
    pub sequence_editor: SequenceEditor,
    pub properties: Properties,
    pub text_editor: TextEditor,
    pub node_editor: NodeEditor,
    pub outliner: Outliner,
    pub info: Info,
    pub preferences: Preferences,
    pub console: Console,
    pub clip_editor: ClipEditor,
    pub topbar: Topbar,
    pub statusbar: Statusbar,
    pub spreadsheet: Spreadsheet,
    pub bone_color_sets: BoneColorSets,
    pub collection_color: CollectionColor,
    pub strip_color: StripColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UserInterface {
    pub theme_user_interface: ThemeUserInterface,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeUserInterface {
    #[serde(rename = "@menu_shadow_fac")]
    pub menu_shadow_fac: Factor,
    #[serde(rename = "@menu_shadow_width")]
    pub menu_shadow_width: u8,
    #[serde(rename = "@icon_alpha")]
    pub icon_alpha: Factor,
    #[serde(rename = "@icon_saturation")]
    pub icon_saturation: Factor,
    #[serde(rename = "@widget_emboss")]
    pub widget_emboss: HexColor,
    #[serde(rename = "@editor_outline")]
    pub editor_outline: HexColor,
    #[serde(rename = "@widget_text_cursor")]
    pub widget_text_cursor: HexColor,
    #[serde(rename = "@panel_roundness")]
    pub panel_roundness: Factor,
    #[serde(rename = "@transparent_checker_primary")]
    pub transparent_checker_primary: HexColor,
    #[serde(rename = "@transparent_checker_secondary")]
    pub transparent_checker_secondary: HexColor,
    #[serde(rename = "@transparent_checker_size")]
    pub transparent_checker_size: u8,
    #[serde(rename = "@axis_x")]
    pub axis_x: HexColor,
    #[serde(rename = "@axis_y")]
    pub axis_y: HexColor,
    #[serde(rename = "@axis_z")]
    pub axis_z: HexColor,
    #[serde(rename = "@gizmo_hi")]
    pub gizmo_hi: HexColor,
    #[serde(rename = "@gizmo_primary")]
    pub gizmo_primary: HexColor,
    #[serde(rename = "@gizmo_secondary")]
    pub gizmo_secondary: HexColor,
    #[serde(rename = "@gizmo_view_align")]
    pub gizmo_view_align: HexColor,
    #[serde(rename = "@gizmo_a")]
    pub gizmo_a: HexColor,
    #[serde(rename = "@gizmo_b")]
    pub gizmo_b: HexColor,
    #[serde(rename = "@icon_scene")]
    pub icon_scene: HexColor,
    #[serde(rename = "@icon_collection")]
    pub icon_collection: HexColor,
    #[serde(rename = "@icon_object")]
    pub icon_object: HexColor,
    #[serde(rename = "@icon_object_data")]
    pub icon_object_data: HexColor,
    #[serde(rename = "@icon_modifier")]
    pub icon_modifier: HexColor,
    #[serde(rename = "@icon_shading")]
    pub icon_shading: HexColor,
    #[serde(rename = "@icon_folder")]
    pub icon_folder: HexColor,
    #[serde(rename = "@icon_border_intensity")]
    pub icon_border_intensity: Factor,
    pub wcol_regular: WcolRegular,
    pub wcol_tool: WcolTool,
    pub wcol_toolbar_item: WcolToolbarItem,
    pub wcol_radio: WcolRadio,
    pub wcol_text: WcolText,
    pub wcol_option: WcolOption,
    pub wcol_toggle: WcolToggle,
    pub wcol_num: WcolNum,
    pub wcol_numslider: WcolNumslider,
    pub wcol_box: WcolBox,
    pub wcol_menu: WcolMenu,
    pub wcol_pulldown: WcolPulldown,
    pub wcol_menu_back: WcolMenuBack,
    pub wcol_pie_menu: WcolPieMenu,
    pub wcol_tooltip: WcolTooltip,
    pub wcol_menu_item: WcolMenuItem,
    pub wcol_scroll: WcolScroll,
    pub wcol_progress: WcolProgress,
    pub wcol_list_item: WcolListItem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wcol_view_item: Option<WcolViewItem>,
    pub wcol_state: WcolState,
    pub wcol_tab: WcolTab,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolRegular {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolTool {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolToolbarItem {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolRadio {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolText {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolOption {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolToggle {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolNum {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolNumslider {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolBox {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolMenu {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolPulldown {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolMenuBack {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolPieMenu {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolTooltip {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolMenuItem {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolScroll {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolProgress {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolListItem {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolViewItem {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolState {
    pub theme_widget_state_colors: ThemeWidgetStateColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WcolTab {
    pub theme_widget_colors: ThemeWidgetColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeWidgetColors {
    #[serde(rename = "@outline")]
    pub outline: HexColor,
    #[serde(rename = "@inner")]
    pub inner: HexColor,
    #[serde(rename = "@inner_sel")]
    pub inner_sel: HexColor,
    #[serde(rename = "@item")]
    pub item: HexColor,
    #[serde(rename = "@text")]
    pub text: HexColor,
    #[serde(rename = "@text_sel")]
    pub text_sel: HexColor,
    #[serde(rename = "@show_shaded")]
    pub show_shaded: Boolean,
    #[serde(rename = "@shadetop")]
    pub shadetop: u8,
    #[serde(rename = "@shadedown")]
    pub shadedown: u8,
    #[serde(rename = "@roundness")]
    pub roundness: Factor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeWidgetStateColors {
    #[serde(rename = "@inner_anim")]
    pub inner_anim: HexColor,
    #[serde(rename = "@inner_anim_sel")]
    pub inner_anim_sel: HexColor,
    #[serde(rename = "@inner_key")]
    pub inner_key: HexColor,
    #[serde(rename = "@inner_key_sel")]
    pub inner_key_sel: HexColor,
    #[serde(rename = "@inner_driven")]
    pub inner_driven: HexColor,
    #[serde(rename = "@inner_driven_sel")]
    pub inner_driven_sel: HexColor,
    #[serde(rename = "@inner_overridden")]
    pub inner_overridden: HexColor,
    #[serde(rename = "@inner_overridden_sel")]
    pub inner_overridden_sel: HexColor,
    #[serde(rename = "@inner_changed")]
    pub inner_changed: HexColor,
    #[serde(rename = "@inner_changed_sel")]
    pub inner_changed_sel: HexColor,
    #[serde(rename = "@blend")]
    pub blend: Factor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct View3d {
    pub theme_view_3_d: ThemeView3D,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeView3D {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@clipping_border_3d")]
    pub clipping_border_3d: HexColor,
    #[serde(rename = "@wire")]
    pub wire: HexColor,
    #[serde(rename = "@wire_edit")]
    pub wire_edit: HexColor,
    #[serde(rename = "@edge_width")]
    pub edge_width: u8,
    #[serde(rename = "@gp_vertex")]
    pub gp_vertex: HexColor,
    #[serde(rename = "@gp_vertex_select")]
    pub gp_vertex_select: HexColor,
    #[serde(rename = "@gp_vertex_size")]
    pub gp_vertex_size: u8,
    #[serde(rename = "@text_grease_pencil")]
    pub text_grease_pencil: HexColor,
    #[serde(rename = "@object_selected")]
    pub object_selected: HexColor,
    #[serde(rename = "@object_active")]
    pub object_active: HexColor,
    #[serde(rename = "@text_keyframe")]
    pub text_keyframe: HexColor,
    #[serde(rename = "@camera")]
    pub camera: HexColor,
    #[serde(rename = "@empty")]
    pub empty: HexColor,
    #[serde(rename = "@light")]
    pub light: HexColor,
    #[serde(rename = "@speaker")]
    pub speaker: HexColor,
    #[serde(rename = "@vertex")]
    pub vertex: HexColor,
    #[serde(rename = "@vertex_select")]
    pub vertex_select: HexColor,
    #[serde(
        rename = "@vertex_active",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertex_active: Option<HexColor>,
    #[serde(rename = "@vertex_size")]
    pub vertex_size: u8,
    #[serde(rename = "@vertex_bevel")]
    pub vertex_bevel: HexColor,
    #[serde(rename = "@vertex_unreferenced")]
    pub vertex_unreferenced: HexColor,
    #[serde(rename = "@edge_select")]
    pub edge_select: HexColor,
    #[serde(
        rename = "@edge_mode_select",
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_mode_select: Option<HexColor>,
    #[serde(rename = "@edge_seam")]
    pub edge_seam: HexColor,
    #[serde(rename = "@edge_sharp")]
    pub edge_sharp: HexColor,
    #[serde(rename = "@edge_crease")]
    pub edge_crease: HexColor,
    #[serde(rename = "@edge_bevel")]
    pub edge_bevel: HexColor,
    #[serde(rename = "@edge_facesel")]
    pub edge_facesel: HexColor,
    #[serde(rename = "@freestyle_edge_mark")]
    pub freestyle_edge_mark: HexColor,
    #[serde(rename = "@face")]
    pub face: HexColor,
    #[serde(rename = "@face_select")]
    pub face_select: HexColor,
    #[serde(
        rename = "@face_mode_select",
        skip_serializing_if = "Option::is_none"
    )]
    pub face_mode_select: Option<HexColor>,
    #[serde(rename = "@face_dot")]
    pub face_dot: HexColor,
    #[serde(rename = "@facedot_size")]
    pub facedot_size: u8,
    #[serde(rename = "@freestyle_face_mark")]
    pub freestyle_face_mark: HexColor,
    #[serde(rename = "@face_retopology")]
    pub face_retopology: HexColor,
    #[serde(rename = "@face_back")]
    pub face_back: HexColor,
    #[serde(rename = "@face_front")]
    pub face_front: HexColor,
    #[serde(rename = "@nurb_uline")]
    pub nurb_uline: HexColor,
    #[serde(rename = "@nurb_vline")]
    pub nurb_vline: HexColor,
    #[serde(rename = "@nurb_sel_uline")]
    pub nurb_sel_uline: HexColor,
    #[serde(rename = "@nurb_sel_vline")]
    pub nurb_sel_vline: HexColor,
    #[serde(rename = "@act_spline")]
    pub act_spline: HexColor,
    #[serde(rename = "@handle_free")]
    pub handle_free: HexColor,
    #[serde(rename = "@handle_auto")]
    pub handle_auto: HexColor,
    #[serde(rename = "@handle_vect")]
    pub handle_vect: HexColor,
    #[serde(rename = "@handle_sel_vect")]
    pub handle_sel_vect: HexColor,
    #[serde(rename = "@handle_align")]
    pub handle_align: HexColor,
    #[serde(rename = "@handle_sel_free")]
    pub handle_sel_free: HexColor,
    #[serde(rename = "@handle_sel_auto")]
    pub handle_sel_auto: HexColor,
    #[serde(rename = "@handle_sel_align")]
    pub handle_sel_align: HexColor,
    #[serde(rename = "@lastsel_point")]
    pub lastsel_point: HexColor,
    #[serde(rename = "@extra_edge_len")]
    pub extra_edge_len: HexColor,
    #[serde(rename = "@extra_edge_angle")]
    pub extra_edge_angle: HexColor,
    #[serde(rename = "@extra_face_angle")]
    pub extra_face_angle: HexColor,
    #[serde(rename = "@extra_face_area")]
    pub extra_face_area: HexColor,
    #[serde(rename = "@editmesh_active")]
    pub editmesh_active: HexColor,
    #[serde(rename = "@normal")]
    pub normal: HexColor,
    #[serde(rename = "@vertex_normal")]
    pub vertex_normal: HexColor,
    #[serde(rename = "@split_normal")]
    pub split_normal: HexColor,
    #[serde(rename = "@bone_pose")]
    pub bone_pose: HexColor,
    #[serde(rename = "@bone_pose_active")]
    pub bone_pose_active: HexColor,
    #[serde(rename = "@bone_solid")]
    pub bone_solid: HexColor,
    #[serde(rename = "@bone_locked_weight")]
    pub bone_locked_weight: HexColor,
    #[serde(rename = "@bundle_solid")]
    pub bundle_solid: HexColor,
    #[serde(rename = "@camera_path")]
    pub camera_path: HexColor,
    #[serde(rename = "@camera_passepartout")]
    pub camera_passepartout: HexColor,
    #[serde(rename = "@skin_root")]
    pub skin_root: HexColor,
    #[serde(rename = "@view_overlay")]
    pub view_overlay: HexColor,
    #[serde(rename = "@transform")]
    pub transform: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@paint_curve_handle")]
    pub paint_curve_handle: HexColor,
    #[serde(rename = "@paint_curve_pivot")]
    pub paint_curve_pivot: HexColor,
    #[serde(rename = "@outline_width")]
    pub outline_width: u8,
    #[serde(rename = "@object_origin_size")]
    pub object_origin_size: u8,
    pub space: Space,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_shelf: Option<AssetShelf>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SpaceGradient {
    pub theme_space_gradient: ThemeSpaceGradient,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AssetShelf {
    pub theme_asset_shelf: ThemeAssetShelf,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeAssetShelf {
    #[serde(rename = "@header_back")]
    pub header_back: HexColor,
    #[serde(rename = "@back")]
    pub back: HexColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct GraphEditor {
    pub theme_graph_editor: ThemeGraphEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeGraphEditor {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@time_scrub_background")]
    pub time_scrub_background: HexColor,
    #[serde(rename = "@time_marker_line")]
    pub time_marker_line: HexColor,
    #[serde(rename = "@time_marker_line_selected")]
    pub time_marker_line_selected: HexColor,
    #[serde(rename = "@window_sliders")]
    pub window_sliders: HexColor,
    #[serde(rename = "@channels_region")]
    pub channels_region: HexColor,
    #[serde(rename = "@dopesheet_channel")]
    pub dopesheet_channel: HexColor,
    #[serde(rename = "@dopesheet_subchannel")]
    pub dopesheet_subchannel: HexColor,
    #[serde(rename = "@channel_group")]
    pub channel_group: HexColor,
    #[serde(rename = "@active_channels_group")]
    pub active_channels_group: HexColor,
    #[serde(rename = "@preview_range")]
    pub preview_range: HexColor,
    #[serde(rename = "@vertex")]
    pub vertex: HexColor,
    #[serde(rename = "@vertex_select")]
    pub vertex_select: HexColor,
    #[serde(rename = "@vertex_active")]
    pub vertex_active: HexColor,
    #[serde(rename = "@vertex_size")]
    pub vertex_size: u8,
    #[serde(rename = "@vertex_bevel")]
    pub vertex_bevel: HexColor,
    #[serde(rename = "@vertex_unreferenced")]
    pub vertex_unreferenced: HexColor,
    #[serde(rename = "@handle_free")]
    pub handle_free: HexColor,
    #[serde(rename = "@handle_auto")]
    pub handle_auto: HexColor,
    #[serde(rename = "@handle_vect")]
    pub handle_vect: HexColor,
    #[serde(rename = "@handle_sel_vect")]
    pub handle_sel_vect: HexColor,
    #[serde(rename = "@handle_align")]
    pub handle_align: HexColor,
    #[serde(rename = "@handle_sel_free")]
    pub handle_sel_free: HexColor,
    #[serde(rename = "@handle_sel_auto")]
    pub handle_sel_auto: HexColor,
    #[serde(rename = "@handle_sel_align")]
    pub handle_sel_align: HexColor,
    #[serde(rename = "@handle_auto_clamped")]
    pub handle_auto_clamped: HexColor,
    #[serde(rename = "@handle_sel_auto_clamped")]
    pub handle_sel_auto_clamped: HexColor,
    #[serde(rename = "@lastsel_point")]
    pub lastsel_point: HexColor,
    #[serde(rename = "@handle_vertex")]
    pub handle_vertex: HexColor,
    #[serde(rename = "@handle_vertex_select")]
    pub handle_vertex_select: HexColor,
    #[serde(rename = "@handle_vertex_size")]
    pub handle_vertex_size: u8,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FileBrowser {
    pub theme_file_browser: ThemeFileBrowser,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeFileBrowser {
    #[serde(rename = "@selected_file")]
    pub selected_file: HexColor,
    #[serde(rename = "@row_alternate")]
    pub row_alternate: HexColor,
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NlaEditor {
    pub theme_n_l_a_editor: ThemeNLAEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeNLAEditor {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@view_sliders")]
    pub view_sliders: HexColor,
    #[serde(rename = "@dopesheet_channel")]
    pub dopesheet_channel: HexColor,
    #[serde(rename = "@nla_track")]
    pub nla_track: HexColor,
    #[serde(rename = "@active_action")]
    pub active_action: HexColor,
    #[serde(rename = "@active_action_unset")]
    pub active_action_unset: HexColor,
    #[serde(rename = "@preview_range")]
    pub preview_range: HexColor,
    #[serde(rename = "@strips")]
    pub strips: HexColor,
    #[serde(rename = "@strips_selected")]
    pub strips_selected: HexColor,
    #[serde(rename = "@transition_strips")]
    pub transition_strips: HexColor,
    #[serde(rename = "@transition_strips_selected")]
    pub transition_strips_selected: HexColor,
    #[serde(rename = "@meta_strips")]
    pub meta_strips: HexColor,
    #[serde(rename = "@meta_strips_selected")]
    pub meta_strips_selected: HexColor,
    #[serde(rename = "@sound_strips")]
    pub sound_strips: HexColor,
    #[serde(rename = "@sound_strips_selected")]
    pub sound_strips_selected: HexColor,
    #[serde(rename = "@tweak")]
    pub tweak: HexColor,
    #[serde(rename = "@tweak_duplicate")]
    pub tweak_duplicate: HexColor,
    #[serde(rename = "@keyframe_border")]
    pub keyframe_border: HexColor,
    #[serde(rename = "@keyframe_border_selected")]
    pub keyframe_border_selected: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@time_scrub_background")]
    pub time_scrub_background: HexColor,
    #[serde(rename = "@time_marker_line")]
    pub time_marker_line: HexColor,
    #[serde(rename = "@time_marker_line_selected")]
    pub time_marker_line_selected: HexColor,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DopesheetEditor {
    pub theme_dope_sheet: ThemeDopeSheet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeDopeSheet {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@time_scrub_background")]
    pub time_scrub_background: HexColor,
    #[serde(rename = "@time_marker_line")]
    pub time_marker_line: HexColor,
    #[serde(rename = "@time_marker_line_selected")]
    pub time_marker_line_selected: HexColor,
    #[serde(rename = "@value_sliders")]
    pub value_sliders: HexColor,
    #[serde(rename = "@view_sliders")]
    pub view_sliders: HexColor,
    #[serde(rename = "@dopesheet_channel")]
    pub dopesheet_channel: HexColor,
    #[serde(rename = "@dopesheet_subchannel")]
    pub dopesheet_subchannel: HexColor,
    #[serde(rename = "@channels")]
    pub channels: HexColor,
    #[serde(rename = "@channels_selected")]
    pub channels_selected: HexColor,
    #[serde(rename = "@channel_group")]
    pub channel_group: HexColor,
    #[serde(rename = "@active_channels_group")]
    pub active_channels_group: HexColor,
    #[serde(rename = "@long_key")]
    pub long_key: HexColor,
    #[serde(rename = "@long_key_selected")]
    pub long_key_selected: HexColor,
    #[serde(rename = "@keyframe")]
    pub keyframe: HexColor,
    #[serde(rename = "@keyframe_selected")]
    pub keyframe_selected: HexColor,
    #[serde(rename = "@keyframe_extreme")]
    pub keyframe_extreme: HexColor,
    #[serde(rename = "@keyframe_extreme_selected")]
    pub keyframe_extreme_selected: HexColor,
    #[serde(rename = "@keyframe_breakdown")]
    pub keyframe_breakdown: HexColor,
    #[serde(rename = "@keyframe_breakdown_selected")]
    pub keyframe_breakdown_selected: HexColor,
    #[serde(rename = "@keyframe_jitter")]
    pub keyframe_jitter: HexColor,
    #[serde(rename = "@keyframe_jitter_selected")]
    pub keyframe_jitter_selected: HexColor,
    #[serde(rename = "@keyframe_movehold")]
    pub keyframe_movehold: HexColor,
    #[serde(rename = "@keyframe_movehold_selected")]
    pub keyframe_movehold_selected: HexColor,
    #[serde(
        rename = "@keyframe_generated",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_generated: Option<HexColor>,
    #[serde(
        rename = "@keyframe_generated_selected",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_generated_selected: Option<HexColor>,
    #[serde(rename = "@keyframe_border")]
    pub keyframe_border: HexColor,
    #[serde(rename = "@keyframe_border_selected")]
    pub keyframe_border_selected: HexColor,
    #[serde(rename = "@keyframe_scale_factor")]
    pub keyframe_scale_factor: f32,
    #[serde(rename = "@summary")]
    pub summary: HexColor,
    #[serde(rename = "@preview_range")]
    pub preview_range: HexColor,
    #[serde(rename = "@interpolation_line")]
    pub interpolation_line: HexColor,
    #[serde(rename = "@simulated_frames")]
    pub simulated_frames: HexColor,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ImageEditor {
    pub theme_image_editor: ThemeImageEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeImageEditor {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@vertex")]
    pub vertex: HexColor,
    #[serde(rename = "@vertex_select")]
    pub vertex_select: HexColor,
    #[serde(
        rename = "@vertex_active",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertex_active: Option<HexColor>,
    #[serde(rename = "@vertex_size")]
    pub vertex_size: u8,
    #[serde(rename = "@vertex_bevel")]
    pub vertex_bevel: HexColor,
    #[serde(rename = "@vertex_unreferenced")]
    pub vertex_unreferenced: HexColor,
    #[serde(rename = "@face")]
    pub face: HexColor,
    #[serde(rename = "@face_select")]
    pub face_select: HexColor,
    #[serde(
        rename = "@face_mode_select",
        skip_serializing_if = "Option::is_none"
    )]
    pub face_mode_select: Option<HexColor>,
    #[serde(rename = "@face_dot")]
    pub face_dot: HexColor,
    #[serde(rename = "@facedot_size")]
    pub facedot_size: u8,
    #[serde(rename = "@freestyle_face_mark")]
    pub freestyle_face_mark: HexColor,
    #[serde(rename = "@face_retopology")]
    pub face_retopology: HexColor,
    #[serde(rename = "@face_back")]
    pub face_back: HexColor,
    #[serde(rename = "@face_front")]
    pub face_front: HexColor,
    #[serde(rename = "@editmesh_active")]
    pub editmesh_active: HexColor,
    #[serde(rename = "@wire_edit")]
    pub wire_edit: HexColor,
    #[serde(rename = "@edge_width")]
    pub edge_width: u8,
    #[serde(rename = "@edge_select")]
    pub edge_select: HexColor,
    #[serde(rename = "@scope_back")]
    pub scope_back: HexColor,
    #[serde(rename = "@preview_stitch_face")]
    pub preview_stitch_face: HexColor,
    #[serde(rename = "@preview_stitch_edge")]
    pub preview_stitch_edge: HexColor,
    #[serde(rename = "@preview_stitch_vert")]
    pub preview_stitch_vert: HexColor,
    #[serde(rename = "@preview_stitch_stitchable")]
    pub preview_stitch_stitchable: HexColor,
    #[serde(rename = "@preview_stitch_unstitchable")]
    pub preview_stitch_unstitchable: HexColor,
    #[serde(rename = "@preview_stitch_active")]
    pub preview_stitch_active: HexColor,
    #[serde(rename = "@uv_shadow")]
    pub uv_shadow: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@metadatabg")]
    pub metadatabg: HexColor,
    #[serde(rename = "@metadatatext")]
    pub metadatatext: HexColor,
    #[serde(rename = "@handle_free")]
    pub handle_free: HexColor,
    #[serde(rename = "@handle_auto")]
    pub handle_auto: HexColor,
    #[serde(rename = "@handle_align")]
    pub handle_align: HexColor,
    #[serde(rename = "@handle_sel_free")]
    pub handle_sel_free: HexColor,
    #[serde(rename = "@handle_sel_auto")]
    pub handle_sel_auto: HexColor,
    #[serde(rename = "@handle_sel_align")]
    pub handle_sel_align: HexColor,
    #[serde(rename = "@handle_auto_clamped")]
    pub handle_auto_clamped: HexColor,
    #[serde(rename = "@handle_sel_auto_clamped")]
    pub handle_sel_auto_clamped: HexColor,
    #[serde(rename = "@handle_vertex")]
    pub handle_vertex: HexColor,
    #[serde(rename = "@handle_vertex_select")]
    pub handle_vertex_select: HexColor,
    #[serde(rename = "@handle_vertex_size")]
    pub handle_vertex_size: u8,
    #[serde(rename = "@paint_curve_handle")]
    pub paint_curve_handle: HexColor,
    #[serde(rename = "@paint_curve_pivot")]
    pub paint_curve_pivot: HexColor,
    pub space: Space,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_shelf: Option<AssetShelf>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SequenceEditor {
    pub theme_sequence_editor: ThemeSequenceEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeSequenceEditor {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@window_sliders")]
    pub window_sliders: HexColor,
    #[serde(rename = "@movie_strip")]
    pub movie_strip: HexColor,
    #[serde(rename = "@movieclip_strip")]
    pub movieclip_strip: HexColor,
    #[serde(rename = "@image_strip")]
    pub image_strip: HexColor,
    #[serde(rename = "@scene_strip")]
    pub scene_strip: HexColor,
    #[serde(rename = "@audio_strip")]
    pub audio_strip: HexColor,
    #[serde(rename = "@effect_strip")]
    pub effect_strip: HexColor,
    #[serde(
        rename = "@transition_strip",
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_strip: Option<HexColor>,
    #[serde(rename = "@color_strip")]
    pub color_strip: HexColor,
    #[serde(rename = "@meta_strip")]
    pub meta_strip: HexColor,
    #[serde(rename = "@mask_strip")]
    pub mask_strip: HexColor,
    #[serde(rename = "@text_strip")]
    pub text_strip: HexColor,
    #[serde(rename = "@active_strip")]
    pub active_strip: HexColor,
    #[serde(rename = "@selected_strip")]
    pub selected_strip: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@time_scrub_background")]
    pub time_scrub_background: HexColor,
    #[serde(rename = "@time_marker_line")]
    pub time_marker_line: HexColor,
    #[serde(rename = "@time_marker_line_selected")]
    pub time_marker_line_selected: HexColor,
    #[serde(rename = "@keyframe")]
    pub keyframe: HexColor,
    #[serde(
        rename = "@keyframe_selected",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_selected: Option<HexColor>,
    #[serde(
        rename = "@keyframe_breakdown",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_breakdown: Option<HexColor>,
    #[serde(
        rename = "@keyframe_breakdown_selected",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_breakdown_selected: Option<HexColor>,
    #[serde(
        rename = "@keyframe_movehold",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_movehold: Option<HexColor>,
    #[serde(
        rename = "@keyframe_movehold_selected",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_movehold_selected: Option<HexColor>,
    #[serde(
        rename = "@keyframe_generated",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_generated: Option<HexColor>,
    #[serde(
        rename = "@keyframe_generated_selected",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_generated_selected: Option<HexColor>,
    #[serde(
        rename = "@keyframe_border",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_border: Option<HexColor>,
    #[serde(
        rename = "@keyframe_border_selected",
        skip_serializing_if = "Option::is_none"
    )]
    pub keyframe_border_selected: Option<HexColor>,
    #[serde(rename = "@draw_action")]
    pub draw_action: HexColor,
    #[serde(rename = "@preview_back")]
    pub preview_back: HexColor,
    #[serde(rename = "@metadatabg")]
    pub metadatabg: HexColor,
    #[serde(rename = "@metadatatext")]
    pub metadatatext: HexColor,
    #[serde(rename = "@preview_range")]
    pub preview_range: HexColor,
    #[serde(rename = "@row_alternate")]
    pub row_alternate: HexColor,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Properties {
    pub theme_properties: ThemeProperties,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeProperties {
    #[serde(rename = "@match")]
    pub r#match: HexColor,
    #[serde(rename = "@active_modifier")]
    pub active_modifier: HexColor,
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TextEditor {
    pub theme_text_editor: ThemeTextEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeTextEditor {
    #[serde(rename = "@line_numbers")]
    pub line_numbers: HexColor,
    #[serde(rename = "@line_numbers_background")]
    pub line_numbers_background: HexColor,
    #[serde(rename = "@selected_text")]
    pub selected_text: HexColor,
    #[serde(rename = "@cursor")]
    pub cursor: HexColor,
    #[serde(rename = "@syntax_builtin")]
    pub syntax_builtin: HexColor,
    #[serde(rename = "@syntax_symbols")]
    pub syntax_symbols: HexColor,
    #[serde(rename = "@syntax_special")]
    pub syntax_special: HexColor,
    #[serde(rename = "@syntax_preprocessor")]
    pub syntax_preprocessor: HexColor,
    #[serde(rename = "@syntax_reserved")]
    pub syntax_reserved: HexColor,
    #[serde(rename = "@syntax_comment")]
    pub syntax_comment: HexColor,
    #[serde(rename = "@syntax_string")]
    pub syntax_string: HexColor,
    #[serde(rename = "@syntax_numbers")]
    pub syntax_numbers: HexColor,
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeEditor {
    pub theme_node_editor: ThemeNodeEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeNodeEditor {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@node_selected")]
    pub node_selected: HexColor,
    #[serde(rename = "@node_active")]
    pub node_active: HexColor,
    #[serde(rename = "@wire")]
    pub wire: HexColor,
    #[serde(rename = "@wire_inner")]
    pub wire_inner: HexColor,
    #[serde(rename = "@wire_select")]
    pub wire_select: HexColor,
    #[serde(rename = "@selected_text")]
    pub selected_text: HexColor,
    #[serde(rename = "@node_backdrop")]
    pub node_backdrop: HexColor,
    #[serde(rename = "@converter_node")]
    pub converter_node: HexColor,
    #[serde(rename = "@color_node")]
    pub color_node: HexColor,
    #[serde(rename = "@group_node")]
    pub group_node: HexColor,
    #[serde(rename = "@group_socket_node")]
    pub group_socket_node: HexColor,
    #[serde(rename = "@frame_node")]
    pub frame_node: HexColor,
    #[serde(rename = "@matte_node")]
    pub matte_node: HexColor,
    #[serde(rename = "@distor_node")]
    pub distor_node: HexColor,
    #[serde(rename = "@noodle_curving")]
    pub noodle_curving: u8,
    #[serde(rename = "@grid_levels")]
    pub grid_levels: u8,
    #[serde(rename = "@dash_alpha")]
    pub dash_alpha: Factor,
    #[serde(rename = "@input_node")]
    pub input_node: HexColor,
    #[serde(rename = "@output_node")]
    pub output_node: HexColor,
    #[serde(rename = "@filter_node")]
    pub filter_node: HexColor,
    #[serde(rename = "@vector_node")]
    pub vector_node: HexColor,
    #[serde(rename = "@texture_node")]
    pub texture_node: HexColor,
    #[serde(rename = "@shader_node")]
    pub shader_node: HexColor,
    #[serde(rename = "@script_node")]
    pub script_node: HexColor,
    #[serde(rename = "@pattern_node")]
    pub pattern_node: HexColor,
    #[serde(rename = "@layout_node")]
    pub layout_node: HexColor,
    #[serde(rename = "@geometry_node")]
    pub geometry_node: HexColor,
    #[serde(rename = "@attribute_node")]
    pub attribute_node: HexColor,
    #[serde(rename = "@simulation_zone")]
    pub simulation_zone: HexColor,
    #[serde(rename = "@repeat_zone", skip_serializing_if = "Option::is_none")]
    pub repeat_zone: Option<HexColor>,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Outliner {
    pub theme_outliner: ThemeOutliner,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeOutliner {
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    theme_info: ThemeInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeInfo {
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Preferences {
    pub theme_preferences: ThemePreferences,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemePreferences {
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Console {
    pub theme_console: ThemeConsole,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeConsole {
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClipEditor {
    pub theme_clip_editor: ThemeClipEditor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeClipEditor {
    #[serde(rename = "@grid")]
    pub grid: HexColor,
    #[serde(rename = "@marker_outline")]
    pub marker_outline: HexColor,
    #[serde(rename = "@marker")]
    pub marker: HexColor,
    #[serde(rename = "@active_marker")]
    pub active_marker: HexColor,
    #[serde(rename = "@selected_marker")]
    pub selected_marker: HexColor,
    #[serde(rename = "@disabled_marker")]
    pub disabled_marker: HexColor,
    #[serde(rename = "@locked_marker")]
    pub locked_marker: HexColor,
    #[serde(rename = "@path_before")]
    pub path_before: HexColor,
    #[serde(rename = "@path_after")]
    pub path_after: HexColor,
    #[serde(rename = "@path_keyframe_before")]
    pub path_keyframe_before: HexColor,
    #[serde(rename = "@path_keyframe_after")]
    pub path_keyframe_after: HexColor,
    #[serde(rename = "@frame_current")]
    pub frame_current: HexColor,
    #[serde(rename = "@time_scrub_background")]
    pub time_scrub_background: HexColor,
    #[serde(rename = "@time_marker_line")]
    pub time_marker_line: HexColor,
    #[serde(rename = "@time_marker_line_selected")]
    pub time_marker_line_selected: HexColor,
    #[serde(rename = "@strips")]
    pub strips: HexColor,
    #[serde(rename = "@strips_selected")]
    pub strips_selected: HexColor,
    #[serde(rename = "@metadatabg")]
    pub metadatabg: HexColor,
    #[serde(rename = "@metadatatext")]
    pub metadatatext: HexColor,
    #[serde(rename = "@handle_free")]
    pub handle_free: HexColor,
    #[serde(rename = "@handle_auto")]
    pub handle_auto: HexColor,
    #[serde(rename = "@handle_align")]
    pub handle_align: HexColor,
    #[serde(rename = "@handle_sel_free")]
    pub handle_sel_free: HexColor,
    #[serde(rename = "@handle_sel_auto")]
    pub handle_sel_auto: HexColor,
    #[serde(rename = "@handle_sel_align")]
    pub handle_sel_align: HexColor,
    #[serde(rename = "@handle_auto_clamped")]
    pub handle_auto_clamped: HexColor,
    #[serde(rename = "@handle_sel_auto_clamped")]
    pub handle_sel_auto_clamped: HexColor,
    #[serde(rename = "@handle_vertex")]
    pub handle_vertex: HexColor,
    #[serde(rename = "@handle_vertex_select")]
    pub handle_vertex_select: HexColor,
    #[serde(rename = "@handle_vertex_size")]
    pub handle_vertex_size: u8,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Topbar {
    pub theme_top_bar: ThemeTopBar,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeTopBar {
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Statusbar {
    pub theme_status_bar: ThemeStatusBar,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeStatusBar {
    pub space: Space,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Spreadsheet {
    pub theme_spreadsheet: ThemeSpreadsheet,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeSpreadsheet {
    #[serde(rename = "@row_alternate")]
    pub row_alternate: HexColor,
    pub space: Space,
    pub space_list: SpaceList,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Space {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_space_generic: Option<ThemeSpaceGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_space_gradient: Option<ThemeSpaceGradient>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeSpaceGeneric {
    #[serde(rename = "@back")]
    pub back: HexColor,
    #[serde(rename = "@title")]
    pub title: HexColor,
    #[serde(rename = "@text")]
    pub text: HexColor,
    #[serde(rename = "@text_hi")]
    pub text_hi: HexColor,
    #[serde(rename = "@header")]
    pub header: HexColor,
    #[serde(rename = "@header_text")]
    pub header_text: HexColor,
    #[serde(rename = "@header_text_hi")]
    pub header_text_hi: HexColor,
    #[serde(rename = "@button")]
    pub button: HexColor,
    #[serde(rename = "@button_title")]
    pub button_title: HexColor,
    #[serde(rename = "@button_text")]
    pub button_text: HexColor,
    #[serde(rename = "@button_text_hi")]
    pub button_text_hi: HexColor,
    #[serde(rename = "@navigation_bar")]
    pub navigation_bar: HexColor,
    #[serde(rename = "@execution_buts")]
    pub execution_buts: HexColor,
    #[serde(rename = "@tab_active")]
    pub tab_active: HexColor,
    #[serde(rename = "@tab_inactive")]
    pub tab_inactive: HexColor,
    #[serde(rename = "@tab_back")]
    pub tab_back: HexColor,
    #[serde(rename = "@tab_outline")]
    pub tab_outline: HexColor,
    pub panelcolors: Panelcolors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeSpaceGradient {
    #[serde(rename = "@title")]
    pub title: HexColor,
    #[serde(rename = "@text")]
    pub text: HexColor,
    #[serde(rename = "@text_hi")]
    pub text_hi: HexColor,
    #[serde(rename = "@header")]
    pub header: HexColor,
    #[serde(rename = "@header_text")]
    pub header_text: HexColor,
    #[serde(rename = "@header_text_hi")]
    pub header_text_hi: HexColor,
    #[serde(rename = "@button")]
    pub button: HexColor,
    #[serde(rename = "@button_title")]
    pub button_title: HexColor,
    #[serde(rename = "@button_text")]
    pub button_text: HexColor,
    #[serde(rename = "@button_text_hi")]
    pub button_text_hi: HexColor,
    #[serde(rename = "@navigation_bar")]
    pub navigation_bar: HexColor,
    #[serde(rename = "@execution_buts")]
    pub execution_buts: HexColor,
    #[serde(rename = "@tab_active")]
    pub tab_active: HexColor,
    #[serde(rename = "@tab_inactive")]
    pub tab_inactive: HexColor,
    #[serde(rename = "@tab_back")]
    pub tab_back: HexColor,
    #[serde(rename = "@tab_outline")]
    pub tab_outline: HexColor,
    pub gradients: Gradients,
    pub panelcolors: Panelcolors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Gradients {
    pub theme_gradient_colors: ThemeGradientColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeGradientColors {
    #[serde(rename = "@background_type")]
    pub background_type: BackGroundType,
    #[serde(rename = "@high_gradient")]
    pub high_gradient: HexColor,
    #[serde(rename = "@gradient")]
    pub gradient: HexColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Panelcolors {
    pub theme_panel_colors: ThemePanelColors,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemePanelColors {
    #[serde(rename = "@header")]
    pub header: HexColor,
    #[serde(rename = "@back")]
    pub back: HexColor,
    #[serde(rename = "@sub_back")]
    pub sub_back: HexColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SpaceList {
    pub theme_space_list_generic: ThemeSpaceListGeneric,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeSpaceListGeneric {
    #[serde(rename = "@list")]
    pub list: HexColor,
    #[serde(rename = "@list_title")]
    pub list_title: HexColor,
    #[serde(rename = "@list_text")]
    pub list_text: HexColor,
    #[serde(rename = "@list_text_hi")]
    pub list_text_hi: HexColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BoneColorSets {
    pub theme_bone_color_set: Vec<ThemeBoneColorSet>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeBoneColorSet {
    #[serde(rename = "@normal")]
    pub normal: HexColor,
    #[serde(rename = "@select")]
    pub select: HexColor,
    #[serde(rename = "@active")]
    pub active: HexColor,
    #[serde(rename = "@show_colored_constraints")]
    pub show_colored_constraints: Boolean,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CollectionColor {
    pub theme_collection_color: Vec<ThemeCollectionColor>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeCollectionColor {
    #[serde(rename = "@color")]
    pub color: HexColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StripColor {
    pub theme_strip_color: Vec<ThemeStripColor>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeStripColor {
    #[serde(rename = "@color")]
    pub color: HexColor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeStyle {
    pub panel_title: PanelTitle,
    pub widget_label: WidgetLabel,
    pub widget: Widget,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PanelTitle {
    pub theme_font_style: ThemeFontStyle,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct WidgetLabel {
    pub theme_font_style: ThemeFontStyle,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Widget {
    pub theme_font_style: ThemeFontStyle,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThemeFontStyle {
    #[serde(rename = "@points")]
    pub points: u8,
    #[serde(
        rename = "@character_weight",
        skip_serializing_if = "Option::is_none"
    )]
    pub character_weight: Option<u16>,
    #[serde(rename = "@shadow")]
    pub shadow: u8,
    #[serde(rename = "@shadow_offset_x")]
    pub shadow_offset_x: i8,
    #[serde(rename = "@shadow_offset_y")]
    pub shadow_offset_y: i8,
    #[serde(rename = "@shadow_alpha")]
    pub shadow_alpha: Factor,
    #[serde(rename = "@shadow_value")]
    pub shadow_value: Factor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Version {
    V3_6,
    V4_0,
    V4_1,
    V4_2,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            Version::V3_6 => "blender_dark_v3.6.xml",
            Version::V4_0 => "blender_dark_v4.0.xml",
            Version::V4_1 => "blender_dark_v4.1.xml",
            Version::V4_2 => "blender_dark_v4.2.xml",
        };
        write!(f, "{version_str}")
    }
}

impl Version {
    pub fn get_theme(&self) -> color_eyre::Result<B3dTheme> {
        let path = self.create_path("themes/original");
        let xml = fs::read_to_string(path)?;

        let xml_de = &mut quick_xml::de::Deserializer::from_str(&xml);
        let result: Result<Bpy, _> = serde_path_to_error::deserialize(xml_de);
        if let Err(err) = &result {
            let path = err.path();
            println!("{path:#?}");
        }

        let bpy: Bpy = quick_xml::de::from_str(&xml)?;
        let theme = B3dTheme {
            bpy,
            version: self.clone(),
        };
        Ok(theme)
    }

    pub fn create_path(&self, parent_dir: &str) -> PathBuf {
        let path = PathBuf::from(parent_dir);
        path.join(self.to_string())
    }
}

impl B3dTheme {
    pub fn create_theme(
        &self,
        destination: &str,
        file_name: &str,
    ) -> color_eyre::Result<B3dTheme> {
        let parent_dir = PathBuf::from(destination);
        fs::create_dir_all(&parent_dir)?;
        let path = parent_dir.join(file_name);

        let xml_serialized = quick_xml::se::to_string(&self.bpy)?;

        let doc = Document::from_str(&xml_serialized)?;
        let xml_serialized = doc.to_string_pretty_with_config(&Config {
            is_pretty: true,
            indent: 2,
            end_pad: 0,
            max_line_length: 0,
            entity_mode: EntityMode::Standard,
            indent_text_nodes: true,
        });

        fs::write(path, &xml_serialized)?;

        let bpy: Bpy = quick_xml::de::from_str(&xml_serialized)?;
        let theme = B3dTheme {
            bpy,
            version: self.version.clone(),
        };

        assert_eq!(self, &theme);

        Ok(theme)
    }
}
