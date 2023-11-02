use std::fs::File;

use wav_io::header::WavHeader;

pub fn read_file(path: &str) -> Result<(WavHeader, Vec<f32>), String>
{
    match File::open(path)
    {
        Ok( file ) =>
        {
            match wav_io::read_from_file(file)
            {
                Ok( wav_file ) =>
                {
                    println!(" Hey I have your wav file! ");
                    //println!(" This is it, {:?}", wav_file);
                    return Ok(wav_file);
                },
                Err(E) =>
                {
                    println!("Not a Wav File [0001]");
                    return Err("Not a Wav File [0001]".to_string());
                }
            }
        },
        Err(E) =>
        {
            println!("No file found [0000]");
            return Err("No file found [0000]".to_string());
        }
    }
}

pub fn write_file(path: &str, wav_file: (WavHeader, Vec<f32>)) 
{
    match File::create(path)
    {
        Ok( mut file ) =>
        {
            match wav_io::write_to_file(& mut file, &wav_file.0, &wav_file.1)
            {
                Ok( wav_file ) =>
                {
                    println!(" Hey I have your wav file! ");
                    //println!(" This is it, {:?}", wav_file);
                },
                Err(E) =>
                {
                    println!("Not a Wav File [0020]");
                }
            }
        },
        Err(E) =>
        {
            println!("No file found [0002]");
        }
    }
}