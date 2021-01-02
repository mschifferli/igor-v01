extern crate portaudio;

const FRAMES: u32 = 64;
const CHANNELS: i32 = 1;
const INTERLEAVED: bool = true;


pub struct Platform {
    pub sample_rate: f64,
    pub frames: u32,
    pub channels: i32,
    pub interleaved: bool,
    pub settings: portaudio::DuplexStreamSettings<f32, f32>,
    pub pa: portaudio::PortAudio
}



impl Platform {
    pub fn new() -> Self {

        let pa: portaudio::PortAudio = portaudio::PortAudio::new().unwrap();
        
        println!("PortAudio:");
        println!("version: {}", pa.version());
        println!("version text: {:?}", pa.version_text());
        println!("host count: {}", pa.host_api_count().unwrap());

        let default_host = pa.default_host_api().unwrap();
        println!("default host: {:#?}", pa.host_api_info(default_host));

        let def_input = pa.default_input_device().unwrap();
        let input_info = pa.device_info(def_input).unwrap();
        println!("Default input device info: {:#?}", &input_info);

        // Construct the input stream parameters.
        let latency = input_info.default_low_input_latency;
        let channels: i32 = CHANNELS; // costrained by input_info.max_output_channels
        let sample_rate: f64 = input_info.default_sample_rate;
        let input_params = portaudio::StreamParameters::<f32>::new(def_input, channels, INTERLEAVED, latency);

        let def_output = pa.default_output_device().unwrap();
        let output_info = pa.device_info(def_output).unwrap();
        println!("Default output device info: {:#?}", &output_info);

        // Construct the output stream parameters.
        let latency = output_info.default_low_output_latency;
        let output_params = portaudio::StreamParameters::<f32>::new(def_output, CHANNELS, INTERLEAVED, latency);

        match pa.is_duplex_format_supported(input_params, output_params, sample_rate) {
              Ok(_) => {}, 
              Err(e) => { eprintln!("Example failed with the following: {:?}", e) }
          }

        // Check that the stream format is supported.
        // pa.is_duplex_format_supported(input_params, output_params, sample_rate).unwrap();

        // Construct the settings with which we'll open our duplex stream.
        let settings = portaudio::DuplexStreamSettings::new(input_params, output_params, sample_rate, FRAMES);

        let platform = Platform { 
            sample_rate: sample_rate,
            frames: FRAMES,
            channels: channels,
            interleaved: INTERLEAVED,
            settings: settings, 
            pa: pa
        };
        platform
    }
}
