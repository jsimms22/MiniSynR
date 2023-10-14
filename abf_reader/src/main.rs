use std::io;
use std::io::prelude::*;
use std::io::{Read, Error, ErrorKind};
use std::fs::File;
use std::env;
use::std::f32;

const CHUNK_SIZE: usize = 512; //all sections of an ABF File are buffered in blocks of 512 bytes

// todo! implement enum
pub enum ABFVersion {
    ABF1,
    ABF2,
}

pub struct ABFHeader {
    // GROUP 1 - File ID and size information
    fFileSignature: u32,        //0
    fFileVersionNumber: f32,    //4
    nOperationMode: u16,        //8
    lActualAcqLength: u32,      //10
    nNumPointsIgnored: u16,     //14
    lActualEpisodes: u32,       //16
    lFileStartDate: u32,        //20
    lFileStartTime: u32,        //24
    lStopwatchTime: u32,        //28
    fHeaderVersionNumber: f32,  //32
    nFileType: u16,             //34
    nMSBinFormat: u16,          //36

    // GROUP 2 - File Structure
    lDataSectionPtr: u32,       //40
    lTagSectionPtr: u32,        //44
    lNumTagEntries: u32,        //48
    lScopeConfigPtr: u32,       //52
    lNumScopes: u32,            //56
    _lDACFilePtr: u32,          //60
    _lDACFileNumEpisodes: u32,  //64
    sUnused68: char,            //68 - 4 chars - unused
    lDeltaArrayPtr: u32,        //72
    lNumDeltas: u32,            //76
    lVoiceTagPtr: u32,          //80
    lVoiceTagEntries: u32,      //84
    lUnused88: u32,             //88 - was number of automatic entries in Notebook section
    lSynchArrayPtr: u32,        //92
    lSynchArraySize: u32,       //96
    nDataFormat: u16,           //100
    nSimultaneousScan: u16,     //102
    lStatisticsConfigPtr: u32,  //104
    lAnnotationSectionPtr: u32, //108
    lNumAnnotations: u32,       //112
    sUnused004: char,           //116 - 2 chars - unused

    // GROUP 3 - Trial Hierarchy Information
    channelCountAcquired: u16,  //118
    nADCNumChannels: u16,       //120
    fADCSampleInterval: f32,    //122
    fADCSecondSampleInterval: f32, //126
    fSynchTimeUnit: f32,        //130
    fSecondsPerRun: f32,        //134
    lNumSamplesPerEpisode: u32, //138
    lPreTriggerSamples: u32,    //142
    lEpisodesPerRun: u32,       //146
    lRunsPerTrial: u32,         //150
    lNumberOfTrials: u32,       //154
    nAveragingMode: u16,        //158
    nUndoRunCount: u16,         //160
    nFirstEpisodeInRun: u16,    //162
    fTriggerThreshold: f32,     //164
    nTriggerSource: u16,        //168
    nTriggerAction: u16,        //170
    nTriggerPolarity: u16,      //172
    fScopeOutputInterval: f32,  //174
    fEpisodeStartToStart: f32,  //178
    fRunStartToStart: f32,      //182
    fTrialStartToStart: f32,    //186
    lAverageCount: u32,         //190
    lClockChange: u32,          //194
    nAutoTriggerStrategy: u16,  //198

    // GROUP 4 - Display Parameters

    // GROUP 5 - Hardware Information

    // GROUP 6 - Environmental Information

    // GROUP 7 - Multi-Channel Information

    // GROUP 8 - Synchronous Timer Outputs

    // GROUP 9 - Epoch Waveform and Pulses

    // GROUP 10 - DAC Output File

    // GROUP 11 - Presweep (conditioning) Pulse Train

    // GROUP 12 - Variable Parameter User List

    // GROUP 13 - Autopeak Measurement

    // GROUP 14 - Channel Arithmetic

    // GROUP 15 - On-Line Subtraction

    // GROUP 16 - Miscellaneous Parameters

    // Extended GROUP 2 - Extended File Structure

    // Extended GROUP 7 - Multi-Channel Information

    // GROUP 17 - Train Parameters

    // Extended GROUP 9 - Extended Epoch Waveform and Pulses

    // Extended GROUP 10 - Extended DAC Output File

    // Extended GORUP 11 - Extended Pre-Sweep (conditioning) Pulse Train

    // Extended GROUP 12 - Extended Variable Parameter User List

    // Extended GROUP 6 - Extended Environmental Information

    // Group
}
pub struct ABFScopeConfig { buffer: Vec<u8>, }
pub struct ABFData { buffer: Vec<u8>, }
pub struct ABFSynchArray { buffer: Vec<u8>, }
pub struct ABFTags { buffer: Vec<u8>, }
pub struct ABFDeltas { buffer: Vec<u8>, }
pub struct DACData { buffer: Vec<u8>, }

impl ABFHeader {
    pub fn build_header(buffer: Vec<u8>) -> ABFHeader {
        todo!()
    }

    pub fn default() { }
}

// abf1 will read initial four characters as: "abf "
fn isAFB1(byte: u8) -> bool { match byte { 32 => true, _ => false, } }
// abf2 will read initial four characters as: "abf2"
fn isAFB2(byte: u8) -> bool { match byte { 50 => true, _ => false, } }

fn read_into_buffer(filepath: &str) -> Vec<u8> {
    let mut f = File::open(&filepath).expect("no file found");
    let metadata = File::metadata(&f).expect("unable to read metadata");

    println!("File size: {:?} (Bytes)", metadata.len());

    let mut buffer = vec![0; metadata.len() as usize];
    match f.read(&mut buffer) {
        Ok(byte) => { /* ... */ },
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("Permissions denied for the provided file, cannot read in bytes.");
            }
            panic!("{}", e);
        }
    }

    println!("file is AFB1: {}", isAFB1(buffer[3]));
    println!("file is AFB2: {}", isAFB2(buffer[3]));

    drop(f); //unnecessary since it only forces the file to drop scope early
    buffer
}



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("We got {:?} arguments: {:?}.", args.len()-1, &args[1..]);

    let filename = &args[1];
    let mut filebuffer = Vec::new();

    filebuffer = read_into_buffer(filename);
/*
    // GROUP 1 - 40 bytes
    let s = std::str::from_utf8(&filebuffer[0..4]).expect("invalid utf-8 sequence.");
    println!("lFileSignature: {}", s); // prints: "AFB "

    let byte_array: [u8; 4] = filebuffer[4..8].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = f32::from_le_bytes(byte_array);
    println!("fFileVersionNumber = {}", value);

    let byte_array: [u8; 2] = filebuffer[8..10].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u16::from_le_bytes(byte_array);
    println!("nOperationMode = {}", value);

    let byte_array: [u8; 4] = filebuffer[10..14].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u32::from_le_bytes(byte_array);
    println!("lActualAcqLength = {}", value);

    let byte_array: [u8; 2] = filebuffer[14..16].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u16::from_le_bytes(byte_array);
    println!("nNumPointsIgnored = {}", value);

    let byte_array: [u8; 4] = filebuffer[16..20].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u32::from_le_bytes(byte_array);
    println!("lActualEpisodes = {}", value);

    let byte_array: [u8; 4] = filebuffer[20..24].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u32::from_le_bytes(byte_array);
    println!("lFileStartDate = {}", value);

    let byte_array: [u8; 4] = filebuffer[24..28].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u32::from_le_bytes(byte_array);
    println!("lFileStartTime = {}", value);

    let byte_array: [u8; 4] = filebuffer[28..32].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u32::from_le_bytes(byte_array);
    println!("lStopwatchTime = {}", value);

    let byte_array: [u8; 4] = filebuffer[32..36].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = f32::from_le_bytes(byte_array);
    println!("fHeaderVersionNumber = {}", value);

    let byte_array: [u8; 2] = filebuffer[36..38].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u16::from_le_bytes(byte_array);
    println!("nFileType = {}", value);

    let byte_array: [u8; 2] = filebuffer[38..40].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u16::from_le_bytes(byte_array);
    println!("nMSBinFormat = {}", value);

    // GROUP 2 - 78 bytes
    let byte_array: [u8; 4] = filebuffer[40..44].try_into().expect("need more bytes");
    //println!("byte array: {:?}", byte_array);
    let value = u32::from_le_bytes(byte_array);
    println!("lDataSectionPtr = {}", value);
*/
}