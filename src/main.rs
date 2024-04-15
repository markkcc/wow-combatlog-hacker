use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use sedregex::ReplaceCommand;
use serde::{Serialize, Deserialize};
use csv::{ReaderBuilder, WriterBuilder};

// Here's a definition of the healing events from the logs that we will manipulate
#[derive(Serialize, Deserialize)]
struct Record {
    date: String,               // 4/10
    time: String,               // 18:47:08.243
    eventname: String,          // SPELL_HEAL or SPELL_PERIODIC_HEAL
    casterid: String,           // Player-1234-8FFFFFFF
    castername: String,         // "Charname-ServerName"
    casterhex: String,          // 0x511 or 0x10511
    castergroup: String,        // 0x0
    targetid: String,           // Player-1234-8FFFFFFF
    targetname: String,         // "Charname-ServerName"
    targethex: String,          // 0x511 or 0x10511
    targetgroup: String,        // 0x0
    spellid: String,            // 425275
    spellname: String,          // "Renew"
    spelltype: String,          // 0x2
    ownerid: String,            // Player-1234-8FFFFFFF
    groupid: String,		// 0000000000000000
    hp: String,                 // 86
    hpmax: String,              // 100
    unknown1: String,           // 0
    unknown2: String,           // 0
    unknown3: String,           // 0
    unknown4: String,           // -1
    unknown5: String,           // 0
    unknown6: String,           // 0
    unknown7: String,           // 0
    coordinatesx: String,       // -7388.14
    coordinatesy: String,       // -4376.14
    zoneid: String,             // 1446
    facingrotation: String,     // 5.3675
    itemlevel: String,          // 50
    heal: u32,                  // 215
    healraw: u32,               // 215
    overheal: String,           // 211
    resisted: String,           // 0
    crit: String,               // nil or 1
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} Charactername-ServerName", args[0]);
        std::process::exit(1);
    }
    let charactername = &args[1];
    let hackfactor = 1.427; // Percentage to increase healing output by
    let mut linecount = 0;
    let (mut totalhealing, mut totalrawhealing) = (0, 0);
    let (mut totalhealing_hacked, mut totalrawhealing_hacked) = (0, 0);
    let (mut currenthealing, mut currentrawhealing): (f32, f32);
    // Define Combat Log input file and modified output file
    let inputfile  = "/Users/mfk/Downloads/WoWCombatLog.txt";
    let outputfile = "/Users/mfk/Downloads/WoWCombatLog-hacked.txt";
    let mut file = OpenOptions::new()
                   .append(true)
                   .open(outputfile)
                   .expect("Unable to open output file");

    // The original Combat Log file lines start with a date,
    // followed by a single space, then a timestamp,
    // followed by double spaces, then the CSV-like data.
    // We use a sed regex to convert the lines to valid CSV.
    let sed_tocsv1   = ReplaceCommand::new("s/ /,/").unwrap();
    let sed_tocsv2   = ReplaceCommand::new("s/  /,/").unwrap();
    let sed_fromcsv1 = ReplaceCommand::new("s/,/ /").unwrap();
    let sed_fromcsv2 = ReplaceCommand::new("s/,/  /").unwrap();

    if let Ok(lines) = read_lines(inputfile) {
        for mut line in lines.flatten() {

            if line.contains(charactername) && (
               line.contains("SPELL_HEAL") ||
               line.contains("SPELL_PERIODIC_HEAL")) {
                  // Convert line to CSV format (from custom format) using sed
		  line = sed_tocsv2.execute(sed_tocsv1.execute(line).to_string()).to_string();
		  // Import line as CSV
		  let mut csvreader = ReaderBuilder::new()
			              .has_headers(false)
			              .flexible(true)
				      .from_reader(line.as_bytes());
		  for result in csvreader.deserialize() {
		    let mut record: Record = result.expect("Misformatted CSV record");
 
		    // Modify healing output values in CSV
		    // add double quotes to match original format
            	    record.castername = format!(r##""{0}""##, record.castername);
            	    record.targetname = format!(r##""{0}""##, record.targetname);
            	    record.spellname  = format!(r##""{0}""##, record.spellname); 

		    // only modify our target character name's healing output 
		    if record.castername.contains(charactername) {
		      currenthealing = record.heal as f32;
            	      currentrawhealing = record.healraw as f32;
            	      totalhealing += currenthealing as i32;
            	      totalrawhealing += currentrawhealing as i32;

            	      // apply hack factor to healing
            	      currenthealing *= hackfactor;
            	      currentrawhealing *= hackfactor;
            	      totalhealing_hacked += currenthealing as i32;
            	      totalrawhealing_hacked += currentrawhealing as i32;

            	      //update record[30] and record[31] with new values
            	      record.heal = currenthealing as u32;
            	      record.healraw = currentrawhealing as u32;
		    }

                    // Export CSV back to line string
		    let mut result_data = Vec::new();
		    {
			// Serialize the record into a vector 
			let mut wtr = WriterBuilder::new()
				      .has_headers(false)
				      .quote(b'#')
				      .double_quote(true)
				      .from_writer(&mut result_data);
			let _ = wtr.serialize(&record);
		    }
		    let mut newline = String::from_utf8(result_data).expect("String conversion error");
		    // Convert CSV back to original format and output it
                    newline = sed_fromcsv2.execute(sed_fromcsv1.execute(newline).to_string()).to_string();
		    // Convert each line to have the Windows new line format (CRLF)
		    // Assuming this is running on macOS, we need to replace \n by \r\n (CRLF)
		    // So we first remove the initial \n, then add the proper line terminator
		    newline.pop();
                    newline = format!("{0}\r\n", newline);
		    file.write(newline.as_bytes()).expect("Error writing to output file");
		  } // end of for results in readcsv.deserialize();
            } else {
                    // line not a heal even by the main character, keep values unmodified
		    // convert each line to have the Windows new line format (CRLF)
		    line = format!("{0}\r\n", line);
                    file.write(line.as_bytes()).expect("Error writing to output file");
            }
	    linecount += 1;
        }
    }
    println!("\nProcessed {} lines.", linecount);
    println!("Healing: {} ==> {}", totalhealing, totalhealing_hacked);
    if totalhealing_hacked != totalrawhealing_hacked { 
        // In case of discrepancy, show it. So far they've always been equal.
        println!("Raw healing: {} ==> {}", totalrawhealing, totalrawhealing_hacked);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
