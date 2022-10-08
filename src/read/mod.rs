use std::fs::File;
use std::io;
use std::fs;
use std::io::Read;

pub fn control() -> StoreFile {
    let fs = input_catch();

    let content = read_file(&fs);

    let data_in_file = text_arrangement(&content);
    
    
    data_in_file
}

fn read_file(file_dir: &String) -> String{
    fs::create_dir_all("drop_here").unwrap();

    let f = format!("drop_here/{}", file_dir.trim());
    
    let mut fs = File::open(&f).unwrap();

    let mut content = String::new();

    fs.read_to_string(&mut content).unwrap();

    content.to_string()
}

fn input_catch() -> String {
    let mut f = String::new();
    println!("Please place the file in the 'drop_here' directory");

    println!("Enter the name of the file in the directory");

    io::stdin().read_line(&mut f)
        .expect("failed to read the line");

    f.trim().to_string()
}

//how to treat with ^2 in the txt invalid
///result: the ^2 sign when you read the sign just disaapeared

///In the file we recieve, the \t are used to represent a new object and the space are used to
///show people the title or things
fn text_arrangement(content: &String) -> StoreFile {
    let mut current_line = String::new();

    let mut null_count = 0;//null in each line

    let mut i = 0;//line number counting

    let mut null_line_flag = true;

    let mut tab_count = 0;

    let mut easy_visual_tab: Vec<i32> = Vec::new();

    let mut return_count = 0;

    let mut no_return_line: Vec<i32> = Vec::new();

    let mut keep_track_struct = 0;

    let mut title_flag = false;

    let mut measurement_flag_count = 0;

    let mut unit_det = false;

    let mut title = String::new();

    let mut measurement: Vec<String> = Vec::new();

    let mut unit: Vec<String> = Vec::new();

    let mut line_data: Vec<f64> = Vec::new();

    let mut data_set: Vec<Vec<f64>> = Vec::new();

    for char in content.chars() {
        if char == '\n' {
            if null_line_flag == false {
                i += 1;
                //println!("at line {}: there is {} space character and there is {} tabs", i , null_count, tab_count);
                //println!("current line = {}\n", current_line);
                if tab_count == 0 {
                } else {
                    easy_visual_tab.push(tab_count);
                }
                //println!("tab_count: {}, null_count :{}", tab_count, null_count);
                let line_type = line_property(&tab_count, &null_count, &current_line);
                match line_type {
                    LineType::Title => {
                        title.push_str(current_line.as_str());
                        title.push(' ');
                        current_line = String::new();
                    },
                    LineType::Description => {
                        title_flag = true;
                        if measurement_flag_count == 1 {
                            measurement.push(current_line);
                            current_line = String::new();
                            unit_det = true;
                            measurement_flag_count += 1;
                        } else {
                            measurement_flag_count += 1;
                        }
                        if measurement_flag_count == 3 {
                            unit.push(current_line);
                            current_line = String::new();
                        }
                        current_line = String::new();
                    },
                    LineType::Information => {
                        line_data.push(parse_float(&current_line));
                        current_line = String::new();
                        data_set.push(line_data);
                        line_data = Vec::new();
                    }
                }
            } else {
            }
            null_count = 0;
            tab_count = 0;
            null_line_flag = true;
        } else if char == '\r' {
            no_return_line.push(i);
            return_count += 1;
        } else if char == '\t' {
            tab_count += 1;
            let descrip = line_property(&tab_count, &null_count, &current_line);
            match descrip {
                LineType::Title => (),
                LineType::Description => {
                    if title_flag == true && unit_det == false {
                        measurement.push(current_line);
                    } else if title_flag == true && unit_det == true {
                        unit.push(current_line);
                    }
                },
                LineType::Information => {
                    line_data.push(parse_float(&current_line));
                },
            }
                current_line = String::new();
        } else if char == ' ' {
            null_count += 1;//set a flag and just check to wipe out the previous one or push current char in this number
        } else {
            null_line_flag = false;
            current_line.push(char);
        }
    }
        println!("title: {}", title);
        println!("measurement :{:?} length:{}", measurement, measurement.len());
        println!("unit: {:?} length: {}", unit, unit.len());
        println!("data_set: {:?} length:{}", data_set, data_set.len());

        for item in data_set.iter() {
            println!("the length for each is {}", item.len());
        }

        for item in data_set.iter(){
            println!("time:{}s       initial_velcocity:{}m/s        after_collision:{}m/s", item[0], item[5], item[2]);

        }
        
        graph_table(&data_set);
    
    let file_ret = StoreFile {
        title: title,
        measurement: measurement,
        unit: unit,
        data: data_set,
    };



    i += 1;
    //println!("at line{}: there is {} space character.           there is only space in this line? => {}", i , null_count, null_line_flag);
    //println!("the number tab count vector: {:?}\n a total of {} lines\nthere is number of return in the file {}\n at line {:?}", easy_visual_tab, easy_visual_tab.len(), return_count, no_return_line);

    /*let dir: Vec<i32> = vec!(1,1,1,1,1,1);
    let dir_1 = dir.clone();

    let mut two_dir: Vec<Vec<i32>> = Vec::new();

    two_dir.push(dir);
    two_dir.push(dir_1);
    println!("{:?}", two_dir[0][1]);*/

    file_ret
}

enum LineType {
    Title,
    Description,
    Information,
}

fn line_property (tab_amount: &i32, space_amount: &i32, line: &String) -> LineType {
    if tab_amount != &0 {
        for char in line.trim().chars() {
            if char != '-' && char != '.' {
                if char.is_numeric() == false {
                    return LineType::Description;
                }
            }
        }
        return LineType::Information;
    }
    LineType::Title
}

fn parse_float(number: &String) -> f64 {
   // println!("number:{}", number);
    let ret_num: f64 = number.trim().parse().unwrap();
    ret_num
}

pub struct StoreFile {
    pub title: String,
    pub measurement: Vec<String>,
    pub unit: Vec<String>,
    pub data: Vec<Vec<f64>>,
}

pub fn graph_table(data: &Vec<Vec<f64>>) {
	let mut max_before = 0.0;
	let mut max_after = 0.0;
	let mut max_before_t = 0.0;
	let mut max_after_t = 0.0; 
	
	for item in data.iter() {
		if max_before <= item[5].abs() {
			max_before = item[5].abs();
			max_before_t = item[0];
		}
		if max_after <= item[2].abs() {
			max_after = item[2].abs();
			max_after_t = item[0];
		}
	}
	println!("max_before:{}, max_after: {}", max_before, max_after);
	
	let mut zt_b: Vec<f64> = Vec::new();//range from zero to ten
	let mut tw_b: Vec<f64> = Vec::new();//range from ten to twenty
	let mut th_b: Vec<f64> = Vec::new();//range from twenty to thirty
	let mut tf_b: Vec<f64> = Vec::new();//range from thirty to forty 
	let mut ff_b: Vec<f64> = Vec::new();//range from 40 to 50
	let mut fs_b: Vec<f64> = Vec::new();//range from 50 to 60
	let mut ss_b: Vec<f64> = Vec::new();//range from 60 to 70
	let mut se_b: Vec<f64> = Vec::new();//range from 70 to 80
	let mut en_b: Vec<f64> = Vec::new();//range from 80 to 90
	let mut nh_b: Vec<f64> = Vec::new();//range from 90 to 100
	
	let mut zt_a: Vec<f64> = Vec::new();//range from zero to ten
	let mut tw_a: Vec<f64> = Vec::new();//range from ten to twenty
	let mut th_a: Vec<f64> = Vec::new();//range from twenty to thirty
	let mut tf_a: Vec<f64> = Vec::new();//range from thirty to forty 
	let mut ff_a: Vec<f64> = Vec::new();//range from 40 to 50
	let mut fs_a: Vec<f64> = Vec::new();//range from 50 to 60
	let mut ss_a: Vec<f64> = Vec::new();//range from 60 to 70
	let mut se_a: Vec<f64> = Vec::new();//range from 70 to 80
	let mut en_a: Vec<f64> = Vec::new();//range from 80 to 90
	let mut nh_a: Vec<f64> = Vec::new();//range from 90 to 100
	
	for item in data.iter() {
		let v_current = item[5].abs();
		let percentage = 100.0 / max_before * v_current;
		let percentage = percentage.floor() as i32;
		//println!("percentage is :{}", percentage);
		match percentage {
			0...9 => {
				zt_b.push(item[0]);
			},
			10...19 => {
				tw_b.push(item[0]);
			},
			20...29 => {
				th_b.push(item[0]);
			},
			30...39 => {
				tf_b.push(item[0]);
			},
			40...49 => {
				ff_b.push(item[0]);
			},
			50...59 => {
				fs_b.push(item[0]);
			},
			60...69 => {
				ss_b.push(item[0]);
			},
			70...79 => {
				se_b.push(item[0]);
			},
			80...89 => {
				en_b.push(item[0]);
			},
			90...100 => {
				nh_b.push(item[0]);
			},
			_ => {
				panic!("wrong io");
			},
		}
	}
	for item in data.iter() {
		let v_current = item[2].abs();
		let percentage = 100.0 / max_after * v_current;
		let percentage = percentage.floor() as i32;
		//println!("percentage after is :{}", percentage);
		match percentage {
			0...9 => {
				zt_a.push(item[0]);
			},
			10...19 => {
				tw_a.push(item[0]);
			},
			20...29 => {
				th_a.push(item[0]);
			},
			30...39 => {
				tf_a.push(item[0]);
			},
			40...49 => {
				ff_a.push(item[0]);
			},
			50...59 => {
				fs_a.push(item[0]);
			},
			60...69 => {
				ss_a.push(item[0]);
			},
			70...79 => {
				se_a.push(item[0]);
			},
			80...89 => {
				en_a.push(item[0]);
			},
			90...100 => {
				nh_a.push(item[0]);
			},
			_ => {
				panic!("wrong io");
			},
		}
	}
	println!("before: 0-10{:?}\n10-20{:?}\n20-30{:?}\n30-40{:?}\n40-50{:?}\n50-60{:?}\n60-70{:?}\n70-80{:?}\n80-90{:?}\n90-100{:?}\n", zt_b, tw_b, th_b, tf_b, ff_b, fs_b, ss_b, se_b, en_b, nh_b);
	println!("after: 0-10{:?}\n10-20{:?}\n20-30{:?}\n30-40{:?}\n40-50{:?}\n50-60{:?}\n60-70{:?}\n70-80{:?}\n80-90{:?}\n90-100{:?}\n", zt_a, tw_a, th_a, tf_a, ff_a, fs_a, ss_a, se_a, en_a, nh_a);
	
	let zt_b = conversion(zt_b);
	let tw_b = conversion(tw_b);
	let th_b = conversion(th_b);
	let tf_b = conversion(tf_b);
	let ff_b = conversion(ff_b);
	let fs_b = conversion(fs_b);
	let ss_b = conversion(ss_b);
	let se_b = conversion(se_b);
	let en_b = conversion(en_b);
	let nh_b = conversion(nh_b);
	println!("before: \n00-10{}\n10-20{}\n20-30{}\n30-40{}\n40-50{}\n50-60{}\n60-70{}\n70-80{}\n80-90{}\n90-100{}\n", zt_b, tw_b, th_b, tf_b, ff_b, fs_b, ss_b, se_b, en_b, nh_b);
	
	let zt_a = conversion(zt_a);
	let tw_a = conversion(tw_a);
	let th_a = conversion(th_a);
	let tf_a = conversion(tf_a);
	let ff_a = conversion(ff_a);
	let fs_a = conversion(fs_a);
	let ss_a = conversion(ss_a);
	let se_a = conversion(se_a);
	let en_a = conversion(en_a);
	let nh_a = conversion(nh_a);
	println!("after: \n00-10{}\n10-20{}\n20-30{}\n30-40{}\n40-50{}\n50-60{}\n60-70{}\n70-80{}\n80-90{}\n90-100{}\n", zt_a, tw_a, th_a, tf_a, ff_a, fs_a, ss_a, se_a, en_a, nh_a);
	
}

fn conversion(time_set: Vec<f64>) -> String {
	let space = ' ';
	let value = '-';
	
	let mut previous = 0.0;
	
	let mut content = String::new();

	
	for item in time_set.iter() {
		let mut times = (item.abs()-previous) / 0.05 - 1.0;
		//println!("times:{}", times);
		while times >= 0.0001 {
			content.push(space);
			times -= 1.0;	
		} 
		content.push(value);
		previous = item.abs().clone();
	}
	content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn see_file() {
        File::create("drop_here/test.txt").unwrap();
        fs::write("drop_here/test.txt", "a").unwrap();
        let sl = read_file(&"test.txt".to_string());
        assert_eq!(sl, "a");
    }

    #[test]
    #[ignore]
    fn check_long_file() {
        let text = read_file(&"exp_test.txt".to_string());

        let mut i = 0;

        let mut content = String::new();

        for char in text.chars() {
            if char == '\n' {  
                i = i + 1;
                println!("line {}: {}", i, content);
                content = String::new();
            } else {
                content.push(char);
            }
        }
        i = i + 1;
        println!("line {}: {}", i, content);
    }

    #[test]
    #[ignore]
    fn check_line_arrangement() {
        control();
    }
}
