use crate::prelude::AudioChannel;

/// Provides functions to control playback of audio: sounds, music etc.
///
pub trait AudioManager {
    /// If true all audio playback is silenced.
    fn is_mute(&self) -> bool;

    /// Set the audio playback is silenced.
    fn set_mute(&self, val: bool);

    /// Begin playback of any specified sound.  Optional parameters allow further control.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique id of the audio media asset.  Can be the className of a loaded asset library.
    /// * `audio_channel_type` - Sounds can be assigned specific channels to allow transformation of groups of related sounds. (optional)
    /// * `loops` - How many times the specified sound should repeat.  Set to -1 for continual loop. (optional, default: 1)
    /// * `start_time` - Time displacement (ms) from the start of the sound file. (optional: default: 0)
    /// * `volume` - Adjusts this sound's amplitude relative to the audioChannel.  0...1: 0 is silent, 1 is full. (optional, default 1.0)
    /// * `pan` - Adjusts this sound's stereo effect relative to the audioChannel.  -1...1: -1 is left channel only, 0 is central, 1 is right channel only. (optional, default: 0.0)
    /// * `is_ignored_if_playing` - If true and this sound is already playing in the specified channel the start request will be skipped.  If false there is a potential for the same sound to play over the top of itself. (optional, default: false)
    /// * `on_complete_callback` - Callback method to execute on sound complete. (optional, default: None)
    fn start(
        &self,
        id: String,
        audio_channel_type: Option<AudioChannel>,
        loops: Option<i32>,
        start_time: Option<i32>,
        volume: Option<f32>,
        pan: Option<f32>,
        is_ignored_if_playing: Option<bool>,
        on_complete_callback: Option<Box<dyn Fn()>>,
    );

    /// End playback of any specified sound.  To stop all sounds on all channels, leave all parameters blank.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique id of the audio media asset intended to be stopped.  If null will stop all sounds on the specific audioChannel. (optional)
    /// * `audio_channel_type` - If specified will only stop sounds assigned to this channel. (optional)
    fn stop(&self, id: Option<String>, audio_channel_type: Option<AudioChannel>);

    /// Adjusts the playback of any specified sound.  To adjust all sounds, ommit id and audio_channel_type.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique id of the audio media asset intended to be transformed.  If null will transform all sounds on the specific audioChannel. (optional)
    /// * `audio_channel_type` - If specified will only transform sounds assigned to this channel. (optional)
    /// * `volume` - Adjusts this sound's amplitude relative to the audioChannel.  0...1: 0 is silent, 1 is full. (optional, default: 1.0)
    /// * `pan` - Adjusts this sound's stereo effect relative to the audioChannel.  -1...1: -1 is left channel only, 0 is central, 1 is right channel only. (optional, default: 0.0)
    /// * `as_relative` - If true will adjust sounds relative to their original transformation.  If false will set them as absolute values. (optional, default: false)
    /// 
    fn transform(
        &self,
        id: Option<String>,
        audio_channel_type: Option<AudioChannel>,
        volume: Option<f32>,
        pan: Option<f32>,
        as_relative: Option<bool>,
    );

    /// Discover if a specified sound is playing.
    /// Returns true if a match is found, otherwise false.
    /// 
    /// * `id` - The unique id of the audio media asset under investigation.  If null will search entire audioChannel for activity. (optional)
    /// * `audio_channel_type` - If specified will only investigate the specified channel.  If ommitted will investigate all channels. (optional)
    ///
    fn is_playing(&self, id: Option<String>, audio_channel_type: Option<AudioChannel>) -> bool;
}
