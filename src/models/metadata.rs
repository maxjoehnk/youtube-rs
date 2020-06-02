use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoMetadataResponse {
    pub player_response: String
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadata {
    pub response_context: ResponseContext,
    pub playability_status: PlayabilityStatus,
    pub streaming_data: Option<StreamingData>,
    pub playback_tracking: Option<PlaybackTracking>,
    pub video_details: VideoDetails,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
    pub player_config: Option<PlayerConfig>,
    pub storyboards: Option<Storyboards>,
    pub microformat: Microformat,
    pub tracking_params: String,
    pub attestation: Option<Attestation>,
    #[serde(default)]
    pub messages: Vec<Message>,
    pub endscreen: Option<Endscreen>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub player_annotations_expanded_renderer: Option<PlayerAnnotationsExpandedRenderer>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAnnotationsExpandedRenderer {
    pub featured_channel: FeaturedChannel,
    pub allow_swipe_dismiss: bool,
    pub annotation_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedChannel {
    pub start_time_ms: String,
    pub end_time_ms: String,
    pub watermark: WatermarkClass,
    pub tracking_params: String,
    pub navigation_endpoint: FeaturedChannelNavigationEndpoint,
    pub channel_name: String,
    pub subscribe_button: SubscribeButtonClass,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedChannelNavigationEndpoint {
    pub click_tracking_params: String,
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseEndpoint {
    pub browse_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeButtonClass {
    pub subscribe_button_renderer: SubscribeButtonRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeButtonRenderer {
    pub button_text: MessageTitle,
    pub subscribed: bool,
    pub enabled: bool,
    #[serde(rename = "type")]
    pub subscribe_button_renderer_type: String,
    pub channel_id: String,
    pub show_preferences: bool,
    pub subscribed_button_text: MessageTitle,
    pub unsubscribed_button_text: MessageTitle,
    pub tracking_params: String,
    pub unsubscribe_button_text: MessageTitle,
    pub service_endpoints: Vec<ServiceEndpoint>,
    pub sign_in_endpoint: Option<SigninEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageTitle {
    pub runs: Vec<Run>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Run {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceEndpoint {
    pub click_tracking_params: String,
    pub subscribe_endpoint: Option<SubscribeEndpoint>,
    pub unsubscribe_endpoint: Option<SubscribeEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeEndpoint {
    pub channel_ids: Vec<String>,
    pub params: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigninEndpoint {
    pub click_tracking_params: String,
    pub web_navigation_endpoint_data: Option<WebNavigationEndpointData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebNavigationEndpointData {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatermarkClass {
    pub thumbnails: Vec<ThumbnailElement>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThumbnailElement {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attestation {
    pub player_attestation_renderer: PlayerAttestationRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAttestationRenderer {
    pub challenge: String,
    pub botguard_data: BotguardData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotguardData {
    pub program: String,
    pub interpreter_url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endscreen {
    pub endscreen_renderer: EndscreenRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndscreenRenderer {
    pub elements: Vec<Element>,
    pub start_ms: String,
    pub tracking_params: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub endscreen_element_renderer: EndscreenElementRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndscreenElementRenderer {
    pub style: EndcardStyle,
    pub image: WatermarkClass,
    pub icon: Option<Icon>,
    pub left: f64,
    pub width: f64,
    pub top: f64,
    pub aspect_ratio: f64,
    pub start_ms: String,
    pub end_ms: String,
    pub title: Title,
    pub metadata: Description,
    pub call_to_action: Option<Description>,
    pub dismiss: Option<Description>,
    pub endpoint: Endpoint,
    pub hovercard_button: Option<SubscribeButtonClass>,
    pub tracking_params: String,
    pub is_subscribe: Option<bool>,
    pub signin_endpoint: Option<SigninEndpoint>,
    pub use_classic_subscribe_button: Option<bool>,
    pub id: String,
    pub video_duration: Option<Description>,
    pub playlist_length: Option<Description>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EndcardStyle {
    Channel,
    Video,
    Playlist,
    Website
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub simple_text: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub click_tracking_params: String,
    pub browse_endpoint: Option<BrowseEndpoint>,
    pub watch_endpoint: Option<WatchEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchEndpoint {
    pub video_id: String,
    pub playlist_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Icon {
    pub thumbnails: Vec<WebNavigationEndpointData>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub accessibility: Accessibility,
    pub simple_text: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessibility {
    pub accessibility_data: AccessibilityData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub mealbar_promo_renderer: MealbarPromoRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MealbarPromoRenderer {
    pub message_texts: Vec<MessageTitle>,
    pub action_button: ActionButtonClass,
    pub dismiss_button: ActionButtonClass,
    pub trigger_condition: String,
    pub style: String,
    pub tracking_params: String,
    pub impression_endpoints: Vec<ImpressionEndpointElement>,
    pub is_visible: bool,
    pub message_title: MessageTitle,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionButtonClass {
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRenderer {
    pub style: String,
    pub size: String,
    pub text: MessageTitle,
    pub service_endpoint: ImpressionEndpointElement,
    pub navigation_endpoint: Option<ButtonRendererNavigationEndpoint>,
    pub tracking_params: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonRendererNavigationEndpoint {
    pub click_tracking_params: String,
    pub url_endpoint: Option<UrlEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UrlEndpoint {
    pub url: String,
    pub target: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpressionEndpointElement {
    pub click_tracking_params: String,
    pub feedback_endpoint: FeedbackEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackEndpoint {
    pub feedback_token: String,
    pub ui_actions: UiActions,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiActions {
    pub hide_enclosing_container: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Microformat {
    pub player_microformat_renderer: PlayerMicroformatRenderer,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMicroformatRenderer {
    pub thumbnail: WatermarkClass,
    pub embed: Embed,
    pub title: Description,
    pub description: Option<Description>,
    pub length_seconds: String,
    pub owner_profile_url: String,
    pub external_channel_id: String,
    pub available_countries: Vec<String>,
    pub is_unlisted: bool,
    pub has_ypc_metadata: bool,
    pub view_count: String,
    pub category: String,
    pub publish_date: String,
    pub owner_channel_name: String,
    pub upload_date: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Embed {
    pub iframe_url: String,
    pub flash_url: String,
    pub width: i64,
    pub height: i64,
    pub flash_secure_url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayabilityStatus {
    pub status: PlaybackStatus,
    #[serde(default)]
    pub playable_in_embed: bool,
    pub context_params: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlaybackStatus {
    Ok,
    Unplayable
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackTracking {
    pub videostats_playback_url: PtrackingUrlClass,
    pub videostats_delayplay_url: PtrackingUrlClass,
    pub videostats_watchtime_url: PtrackingUrlClass,
    pub ptracking_url: PtrackingUrlClass,
    pub qoe_url: PtrackingUrlClass,
    pub set_awesome_url: AtrUrlClass,
    pub atr_url: AtrUrlClass,
    pub youtube_remarketing_url: Option<AtrUrlClass>,
    pub google_remarketing_url: Option<AtrUrlClass>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtrUrlClass {
    pub base_url: String,
    pub elapsed_media_time_seconds: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtrackingUrlClass {
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerConfig {
    pub audio_config: AudioConfig,
    pub stream_selection_config: Option<StreamSelectionConfig>,
    pub media_common_config: MediaCommonConfig,
    pub web_player_config: WebPlayerConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioConfig {
    pub loudness_db: Option<f64>,
    pub perceptual_loudness_db: Option<f64>,
    pub enable_per_format_loudness: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCommonConfig {
    pub dynamic_readahead_config: DynamicReadaheadConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicReadaheadConfig {
    pub max_read_ahead_media_time_ms: i64,
    pub min_read_ahead_media_time_ms: i64,
    pub read_ahead_growth_rate_ms: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSelectionConfig {
    pub max_bitrate: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebPlayerConfig {
    pub web_player_actions_porting: WebPlayerActionsPorting,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebPlayerActionsPorting {
    pub get_share_panel_command: GetSharePanelCommand,
    pub subscribe_command: SubscribeCommand,
    pub unsubscribe_command: UnsubscribeCommand,
    pub add_to_watch_later_command: AddToWatchLaterCommand,
    pub remove_from_watch_later_command: RemoveFromWatchLaterCommand,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToWatchLaterCommand {
    pub click_tracking_params: String,
    pub playlist_edit_endpoint: AddToWatchLaterCommandPlaylistEditEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddToWatchLaterCommandPlaylistEditEndpoint {
    pub playlist_id: String,
    pub actions: Vec<PurpleAction>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleAction {
    pub added_video_id: String,
    pub action: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSharePanelCommand {
    pub click_tracking_params: String,
    pub web_player_share_entity_service_endpoint: WebPlayerShareEntityServiceEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebPlayerShareEntityServiceEndpoint {
    pub serialized_share_entity: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromWatchLaterCommand {
    pub click_tracking_params: String,
    pub playlist_edit_endpoint: RemoveFromWatchLaterCommandPlaylistEditEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromWatchLaterCommandPlaylistEditEndpoint {
    pub playlist_id: String,
    pub actions: Vec<FluffyAction>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyAction {
    pub action: String,
    pub removed_video_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeCommand {
    pub click_tracking_params: String,
    pub subscribe_endpoint: SubscribeEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeCommand {
    pub click_tracking_params: String,
    pub unsubscribe_endpoint: SubscribeEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub service_tracking_params: Vec<ServiceTrackingParam>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceTrackingParam {
    pub service: String,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Storyboards {
    pub player_storyboard_spec_renderer: PlayerStoryboardSpecRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStoryboardSpecRenderer {
    pub spec: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    pub expires_in_seconds: String,
    pub formats: Vec<Format>,
    pub adaptive_formats: Vec<Format>,
    pub probe_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub itag: i64,
    pub mime_type: String,
    pub bitrate: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub init_range: Option<Range>,
    pub index_range: Option<Range>,
    pub last_modified: String,
    pub content_length: Option<String>,
    pub quality: String,
    pub fps: Option<i64>,
    pub quality_label: Option<String>,
    pub projection_type: ProjectionType,
    pub average_bitrate: Option<i64>,
    pub approx_duration_ms: Option<String>,
    pub cipher: Option<String>,
    pub color_info: Option<ColorInfo>,
    pub high_replication: Option<bool>,
    pub audio_quality: Option<String>,
    pub audio_sample_rate: Option<String>,
    pub audio_channels: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum VideoQuality {
    Small,
    Medium,
    Large
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    Low,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    Medium
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
    pub primaries: String,
    pub transfer_characteristics: String,
    pub matrix_coefficients: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub video_id: String,
    pub title: String,
    pub length_seconds: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub channel_id: String,
    pub is_owner_viewing: bool,
    pub short_description: String,
    pub is_crawlable: bool,
    pub thumbnail: WatermarkClass,
    pub average_rating: f64,
    pub allow_ratings: bool,
    pub view_count: String,
    pub author: String,
    pub is_private: bool,
    pub is_unplugged_corpus: bool,
    pub is_live_content: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProjectionType {
    Rectangular,
}
