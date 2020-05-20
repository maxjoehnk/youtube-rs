use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct VideoMetadataResponse {
    pub player_response: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoMetadata {
    #[serde(rename = "responseContext")]
    pub response_context: ResponseContext,
    #[serde(rename = "playabilityStatus")]
    pub playability_status: PlayabilityStatus,
    #[serde(rename = "streamingData")]
    pub streaming_data: Option<StreamingData>,
    #[serde(rename = "playbackTracking")]
    pub playback_tracking: Option<PlaybackTracking>,
    #[serde(rename = "videoDetails")]
    pub video_details: VideoDetails,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
    #[serde(rename = "playerConfig")]
    pub player_config: Option<PlayerConfig>,
    pub storyboards: Option<Storyboards>,
    pub microformat: Microformat,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    pub attestation: Option<Attestation>,
    #[serde(default)]
    pub messages: Vec<Message>,
    pub endscreen: Option<Endscreen>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Annotation {
    #[serde(rename = "playerAnnotationsExpandedRenderer")]
    pub player_annotations_expanded_renderer: PlayerAnnotationsExpandedRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerAnnotationsExpandedRenderer {
    #[serde(rename = "featuredChannel")]
    pub featured_channel: FeaturedChannel,
    #[serde(rename = "allowSwipeDismiss")]
    pub allow_swipe_dismiss: bool,
    #[serde(rename = "annotationId")]
    pub annotation_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeaturedChannel {
    #[serde(rename = "startTimeMs")]
    pub start_time_ms: String,
    #[serde(rename = "endTimeMs")]
    pub end_time_ms: String,
    pub watermark: WatermarkClass,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: FeaturedChannelNavigationEndpoint,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "subscribeButton")]
    pub subscribe_button: SubscribeButtonClass,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeaturedChannelNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "browseEndpoint")]
    pub browse_endpoint: BrowseEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BrowseEndpoint {
    #[serde(rename = "browseId")]
    pub browse_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeButtonClass {
    #[serde(rename = "subscribeButtonRenderer")]
    pub subscribe_button_renderer: SubscribeButtonRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeButtonRenderer {
    #[serde(rename = "buttonText")]
    pub button_text: MessageTitle,
    pub subscribed: bool,
    pub enabled: bool,
    #[serde(rename = "type")]
    pub subscribe_button_renderer_type: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "showPreferences")]
    pub show_preferences: bool,
    #[serde(rename = "subscribedButtonText")]
    pub subscribed_button_text: MessageTitle,
    #[serde(rename = "unsubscribedButtonText")]
    pub unsubscribed_button_text: MessageTitle,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "unsubscribeButtonText")]
    pub unsubscribe_button_text: MessageTitle,
    #[serde(rename = "serviceEndpoints")]
    pub service_endpoints: Vec<ServiceEndpoint>,
    #[serde(rename = "signInEndpoint")]
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
pub struct ServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "subscribeEndpoint")]
    pub subscribe_endpoint: Option<SubscribeEndpoint>,
    #[serde(rename = "unsubscribeEndpoint")]
    pub unsubscribe_endpoint: Option<SubscribeEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeEndpoint {
    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<String>,
    pub params: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SigninEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "webNavigationEndpointData")]
    pub web_navigation_endpoint_data: WebNavigationEndpointData,
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
pub struct Attestation {
    #[serde(rename = "playerAttestationRenderer")]
    pub player_attestation_renderer: PlayerAttestationRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerAttestationRenderer {
    pub challenge: String,
    #[serde(rename = "botguardData")]
    pub botguard_data: BotguardData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BotguardData {
    pub program: String,
    #[serde(rename = "interpreterUrl")]
    pub interpreter_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Endscreen {
    #[serde(rename = "endscreenRenderer")]
    pub endscreen_renderer: EndscreenRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EndscreenRenderer {
    pub elements: Vec<Element>,
    #[serde(rename = "startMs")]
    pub start_ms: String,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Element {
    #[serde(rename = "endscreenElementRenderer")]
    pub endscreen_element_renderer: EndscreenElementRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EndscreenElementRenderer {
    pub style: EndcardStyle,
    pub image: WatermarkClass,
    pub icon: Option<Icon>,
    pub left: f64,
    pub width: f64,
    pub top: f64,
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: f64,
    #[serde(rename = "startMs")]
    pub start_ms: String,
    #[serde(rename = "endMs")]
    pub end_ms: String,
    pub title: Title,
    pub metadata: Description,
    #[serde(rename = "callToAction")]
    pub call_to_action: Option<Description>,
    pub dismiss: Option<Description>,
    pub endpoint: Endpoint,
    #[serde(rename = "hovercardButton")]
    pub hovercard_button: Option<SubscribeButtonClass>,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "isSubscribe")]
    pub is_subscribe: Option<bool>,
    #[serde(rename = "signinEndpoint")]
    pub signin_endpoint: Option<SigninEndpoint>,
    #[serde(rename = "useClassicSubscribeButton")]
    pub use_classic_subscribe_button: Option<bool>,
    pub id: String,
    #[serde(rename = "videoDuration")]
    pub video_duration: Option<Description>,
    #[serde(rename = "playlistLength")]
    pub playlist_length: Option<Description>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EndcardStyle {
    Channel,
    Video,
    Playlist
}

#[derive(Debug, Clone, Deserialize)]
pub struct Description {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "browseEndpoint")]
    pub browse_endpoint: Option<BrowseEndpoint>,
    #[serde(rename = "watchEndpoint")]
    pub watch_endpoint: Option<WatchEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WatchEndpoint {
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "playlistId")]
    pub playlist_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Icon {
    pub thumbnails: Vec<WebNavigationEndpointData>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Title {
    pub accessibility: Accessibility,
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Accessibility {
    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessibilityData {
    pub label: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    #[serde(rename = "mealbarPromoRenderer")]
    pub mealbar_promo_renderer: MealbarPromoRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MealbarPromoRenderer {
    #[serde(rename = "messageTexts")]
    pub message_texts: Vec<MessageTitle>,
    #[serde(rename = "actionButton")]
    pub action_button: ActionButtonClass,
    #[serde(rename = "dismissButton")]
    pub dismiss_button: ActionButtonClass,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    pub style: String,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "impressionEndpoints")]
    pub impression_endpoints: Vec<ImpressionEndpointElement>,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    #[serde(rename = "messageTitle")]
    pub message_title: MessageTitle,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActionButtonClass {
    #[serde(rename = "buttonRenderer")]
    pub button_renderer: ButtonRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ButtonRenderer {
    pub style: String,
    pub size: String,
    pub text: MessageTitle,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: ImpressionEndpointElement,
    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: Option<ButtonRendererNavigationEndpoint>,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ButtonRendererNavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "urlEndpoint")]
    pub url_endpoint: UrlEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UrlEndpoint {
    pub url: String,
    pub target: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImpressionEndpointElement {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "feedbackEndpoint")]
    pub feedback_endpoint: FeedbackEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeedbackEndpoint {
    #[serde(rename = "feedbackToken")]
    pub feedback_token: String,
    #[serde(rename = "uiActions")]
    pub ui_actions: UiActions,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UiActions {
    #[serde(rename = "hideEnclosingContainer")]
    pub hide_enclosing_container: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Microformat {
    #[serde(rename = "playerMicroformatRenderer")]
    pub player_microformat_renderer: PlayerMicroformatRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerMicroformatRenderer {
    pub thumbnail: WatermarkClass,
    pub embed: Embed,
    pub title: Description,
    pub description: Description,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: String,
    #[serde(rename = "ownerProfileUrl")]
    pub owner_profile_url: String,
    #[serde(rename = "externalChannelId")]
    pub external_channel_id: String,
    #[serde(rename = "availableCountries")]
    pub available_countries: Vec<String>,
    #[serde(rename = "isUnlisted")]
    pub is_unlisted: bool,
    #[serde(rename = "hasYpcMetadata")]
    pub has_ypc_metadata: bool,
    #[serde(rename = "viewCount")]
    pub view_count: String,
    pub category: String,
    #[serde(rename = "publishDate")]
    pub publish_date: String,
    #[serde(rename = "ownerChannelName")]
    pub owner_channel_name: String,
    #[serde(rename = "uploadDate")]
    pub upload_date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Embed {
    #[serde(rename = "iframeUrl")]
    pub iframe_url: String,
    #[serde(rename = "flashUrl")]
    pub flash_url: String,
    pub width: i64,
    pub height: i64,
    #[serde(rename = "flashSecureUrl")]
    pub flash_secure_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayabilityStatus {
    pub status: PlaybackStatus,
    #[serde(rename = "playableInEmbed", default)]
    pub playable_in_embed: bool,
    #[serde(rename = "contextParams")]
    pub context_params: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlaybackStatus {
    Ok,
    Unplayable
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaybackTracking {
    #[serde(rename = "videostatsPlaybackUrl")]
    pub videostats_playback_url: PtrackingUrlClass,
    #[serde(rename = "videostatsDelayplayUrl")]
    pub videostats_delayplay_url: PtrackingUrlClass,
    #[serde(rename = "videostatsWatchtimeUrl")]
    pub videostats_watchtime_url: PtrackingUrlClass,
    #[serde(rename = "ptrackingUrl")]
    pub ptracking_url: PtrackingUrlClass,
    #[serde(rename = "qoeUrl")]
    pub qoe_url: PtrackingUrlClass,
    #[serde(rename = "setAwesomeUrl")]
    pub set_awesome_url: AtrUrlClass,
    #[serde(rename = "atrUrl")]
    pub atr_url: AtrUrlClass,
    #[serde(rename = "youtubeRemarketingUrl")]
    pub youtube_remarketing_url: Option<AtrUrlClass>,
    #[serde(rename = "googleRemarketingUrl")]
    pub google_remarketing_url: Option<AtrUrlClass>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtrUrlClass {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "elapsedMediaTimeSeconds")]
    pub elapsed_media_time_seconds: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PtrackingUrlClass {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerConfig {
    #[serde(rename = "audioConfig")]
    pub audio_config: AudioConfig,
    #[serde(rename = "streamSelectionConfig")]
    pub stream_selection_config: Option<StreamSelectionConfig>,
    #[serde(rename = "mediaCommonConfig")]
    pub media_common_config: MediaCommonConfig,
    #[serde(rename = "webPlayerConfig")]
    pub web_player_config: WebPlayerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioConfig {
    #[serde(rename = "loudnessDb")]
    pub loudness_db: Option<f64>,
    #[serde(rename = "perceptualLoudnessDb")]
    pub perceptual_loudness_db: Option<f64>,
    #[serde(rename = "enablePerFormatLoudness")]
    pub enable_per_format_loudness: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaCommonConfig {
    #[serde(rename = "dynamicReadaheadConfig")]
    pub dynamic_readahead_config: DynamicReadaheadConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DynamicReadaheadConfig {
    #[serde(rename = "maxReadAheadMediaTimeMs")]
    pub max_read_ahead_media_time_ms: i64,
    #[serde(rename = "minReadAheadMediaTimeMs")]
    pub min_read_ahead_media_time_ms: i64,
    #[serde(rename = "readAheadGrowthRateMs")]
    pub read_ahead_growth_rate_ms: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamSelectionConfig {
    #[serde(rename = "maxBitrate")]
    pub max_bitrate: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebPlayerConfig {
    #[serde(rename = "webPlayerActionsPorting")]
    pub web_player_actions_porting: WebPlayerActionsPorting,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebPlayerActionsPorting {
    #[serde(rename = "getSharePanelCommand")]
    pub get_share_panel_command: GetSharePanelCommand,
    #[serde(rename = "subscribeCommand")]
    pub subscribe_command: SubscribeCommand,
    #[serde(rename = "unsubscribeCommand")]
    pub unsubscribe_command: UnsubscribeCommand,
    #[serde(rename = "addToWatchLaterCommand")]
    pub add_to_watch_later_command: AddToWatchLaterCommand,
    #[serde(rename = "removeFromWatchLaterCommand")]
    pub remove_from_watch_later_command: RemoveFromWatchLaterCommand,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddToWatchLaterCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "playlistEditEndpoint")]
    pub playlist_edit_endpoint: AddToWatchLaterCommandPlaylistEditEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddToWatchLaterCommandPlaylistEditEndpoint {
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    pub actions: Vec<PurpleAction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PurpleAction {
    #[serde(rename = "addedVideoId")]
    pub added_video_id: String,
    pub action: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetSharePanelCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "webPlayerShareEntityServiceEndpoint")]
    pub web_player_share_entity_service_endpoint: WebPlayerShareEntityServiceEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebPlayerShareEntityServiceEndpoint {
    #[serde(rename = "serializedShareEntity")]
    pub serialized_share_entity: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveFromWatchLaterCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "playlistEditEndpoint")]
    pub playlist_edit_endpoint: RemoveFromWatchLaterCommandPlaylistEditEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoveFromWatchLaterCommandPlaylistEditEndpoint {
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    pub actions: Vec<FluffyAction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FluffyAction {
    pub action: String,
    #[serde(rename = "removedVideoId")]
    pub removed_video_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "subscribeEndpoint")]
    pub subscribe_endpoint: SubscribeEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnsubscribeCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "unsubscribeEndpoint")]
    pub unsubscribe_endpoint: SubscribeEndpoint,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseContext {
    #[serde(rename = "serviceTrackingParams")]
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
pub struct Storyboards {
    #[serde(rename = "playerStoryboardSpecRenderer")]
    pub player_storyboard_spec_renderer: PlayerStoryboardSpecRenderer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlayerStoryboardSpecRenderer {
    pub spec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: String,
    pub formats: Vec<Format>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<Format>,
    #[serde(rename = "probeUrl")]
    pub probe_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Format {
    pub itag: i64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub bitrate: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    #[serde(rename = "initRange")]
    pub init_range: Option<Range>,
    #[serde(rename = "indexRange")]
    pub index_range: Option<Range>,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "contentLength")]
    pub content_length: Option<String>,
    pub quality: String,
    pub fps: Option<i64>,
    #[serde(rename = "qualityLabel")]
    pub quality_label: Option<String>,
    #[serde(rename = "projectionType")]
    pub projection_type: ProjectionType,
    #[serde(rename = "averageBitrate")]
    pub average_bitrate: Option<i64>,
    #[serde(rename = "approxDurationMs")]
    pub approx_duration_ms: Option<String>,
    pub cipher: Option<String>,
    #[serde(rename = "colorInfo")]
    pub color_info: Option<ColorInfo>,
    #[serde(rename = "highReplication")]
    pub high_replication: Option<bool>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: Option<String>,
    #[serde(rename = "audioSampleRate")]
    pub audio_sample_rate: Option<String>,
    #[serde(rename = "audioChannels")]
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
pub struct ColorInfo {
    pub primaries: String,
    #[serde(rename = "transferCharacteristics")]
    pub transfer_characteristics: String,
    #[serde(rename = "matrixCoefficients")]
    pub matrix_coefficients: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Range {
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub title: String,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "isOwnerViewing")]
    pub is_owner_viewing: bool,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "isCrawlable")]
    pub is_crawlable: bool,
    pub thumbnail: WatermarkClass,
    #[serde(rename = "averageRating")]
    pub average_rating: f64,
    #[serde(rename = "allowRatings")]
    pub allow_ratings: bool,
    #[serde(rename = "viewCount")]
    pub view_count: String,
    pub author: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "isUnpluggedCorpus")]
    pub is_unplugged_corpus: bool,
    #[serde(rename = "isLiveContent")]
    pub is_live_content: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ProjectionType {
    #[serde(rename = "RECTANGULAR")]
    Rectangular,
}
