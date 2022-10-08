use super::read;
use std::io;

fn call_this() {
    let q = read::control();
    //println!("data:{:?}", q.data);
    let index_set = user_preference(&q.measurement);
    println!("time[{}]; v_before[{}]; v_after[{}];", index_set.time, index_set.v_before, index_set.v_after);

    let time = find_collide_time_range(&q.data, &index_set.v_after);

    let detail_info = before_after_collision_speed(&q.data, &time, &index_set.v_before, &index_set.v_after);

    let momentum_percentage = BeAfCollsion::momentum_percentage(&detail_info);

    let energy_kinetic_percentage = BeAfCollsion::energy_kinetic_percentage(&detail_info);

    let elastic_cmp_inelastic_velocity_percentage = BeAfCollsion::elastic_cmp_inelastic_velocity_percentage(&detail_info);

    let time_difference = BeAfCollsion::time_difference(&detail_info);
}

fn user_preference(measurement: &Vec<String>) -> InfoIndex {
    let mut i = 0;
    println!("according to the table you provide us:\n There are a few things the charts is measuring");
    for item in measurement.iter() {
        println!("#{}: {},", i, item);
        i += 1;
    }
    println!("please index in the following sequence:\ntime,\nvelocity before collision,\nvelocity after collision");
    
    println!("type in number of coloum for time");
    let item_code = ask_io(6);

    println!("type in the number of coloum for velocity before collision");
    let v1_code = ask_io(6);

    println!("type in the number of coloum for velocity after collision");
    let v2_code = ask_io(6);

    let index_set = InfoIndex {
        time: item_code,
        v_before: v1_code,
        v_after: v2_code,
    };
    index_set
}

struct InfoIndex{
    time: u8,
    v_before: u8,
    v_after: u8,
}

fn ask_io (max_index: u8) -> u8 {
    let mut no_attempt = true;
    let mut content = String::new();
    let mut index: u8 = 0;
    loop {
        if no_attempt == false {
            println!("the index should be small than {} and bigger than 0", max_index);
        }
        io::stdin().read_line(&mut content)
            .expect("failed to read the line");
        no_attempt = false;
        index = content.trim().parse().unwrap();
        if index <= max_index && index >= 0 {
            break;
        }
    }
    index
}

fn find_collide_time_range(data_set: &Vec<Vec<f64>>, v_after: &u8) -> f64 {
    let mut no_vaild_data = true;
    let mut capture_time: f64 = 0.0;
    let mut v2_before: f64 = 0.0;
    let v_index: usize = *v_after as usize; 
    let mut percentage: f64 = 0.0;
    for item in data_set {
        percentage = 100.0/item[v_index].abs()*v2_before.abs();

        if percentage >= 15.0 && item[v_index].abs() >= 0.1 {
            capture_time = item[0];
            no_vaild_data == false;
            break;
        }
        v2_before = item[v_index];
    }

    println!("the percentage_difference is: {}", percentage);

    println!("captured_time: {}", capture_time);
    
    capture_time
}

struct BeAfCollsion {
    ma: f64,
    mb: f64,
    before_time: f64,
    before_va: f64,
    before_vb: f64,
    after_time: f64,
    after_va: f64,
    after_vb: f64,
}

impl BeAfCollsion{
    fn momentum_percentage(&self) -> f64 {
        let mometum_before = self.ma * self.before_va + self.mb * self.before_vb;
        let mometum_after = self.ma * self.after_va + self.mb * self.after_vb;
        let percentage = mometum_after / mometum_before * 100.0;
        println!("the percentage of mometum {}", percentage);
        percentage
    }

    fn energy_kinetic_percentage(&self) -> f64 {
        let energy_before = 0.5 * self.ma * self.before_va * self.before_va + 0.5 * self.mb * self.before_vb * self.before_vb;
        let energy_after = 0.5 * self.ma * self.after_va * self.after_va + 0.5 * self.mb * self.after_vb * self.after_vb;
        let percentage = energy_after / energy_before * 100.0;
        println!("the percentage of kinetic energy {}", percentage);
        percentage
    }

    fn elastic_cmp_inelastic_velocity_percentage(&self) -> IdealTwoCase {
        let mometum_before = self.ma * self.before_va + self.mb * self.before_vb;
        let energy_before = 0.5 * self.ma * self.before_va * self.before_va + 0.5 * self.mb * self.before_vb * self.before_vb;

        let mut vb_prime_coefficient = 0.0;

        let mometum_constant = mometum_before / self.ma;

        let vb_prime_coefficient = self.mb / self.ma;

        let vb_square_coefficient = vb_prime_coefficient * vb_prime_coefficient * 0.5 * self.ma + 0.5 * self.mb;

        let vb_coefficient = mometum_constant * vb_prime_coefficient * self.ma;

        let energy_coefficient = 0.5 * self.ma * mometum_constant * mometum_constant - energy_before;

        let delta = 4.0 * energy_coefficient * vb_square_coefficient;

        let square = vb_coefficient* vb_coefficient;

        let delta = square - delta;
        if delta < 0.0 {
            panic!("delta of quadratic is smaller than 0 recheck to experiment invalid");
        }
        
        //println!("\n\nin the quadratic the a is {}, the b is {} the c is {}\n\n", vb_square_coefficient, vb_coefficient, energy_coefficient);
        

        let vb1 = (vb_coefficient + delta.sqrt()) / (2.0 * vb_square_coefficient);

        let vb2 = (vb_coefficient - delta.sqrt()) / (2.0 * vb_square_coefficient);

        let va1 = mometum_constant - vb_prime_coefficient * vb1;

        let va2 = mometum_constant - vb_prime_coefficient * vb2;

        let velocity_percentage1 = self.after_vb / vb1;

        let velocity_percentage2 = self.after_vb / vb2;

        println!("the idead elastic collsion will have a velocity of {} or {}\n in our experiment the velocity is {} \n the the percentage of the elasticity is {} or {}", vb1, vb2, self.after_vb, velocity_percentage1, velocity_percentage2);

        let ret_package = IdealTwoCase {
            case1_va: va1,
            case1_vb: vb1,
            case1_percentage: velocity_percentage1,
            case2_va: va2,
            case2_vb: vb2,
            case2_percentage: velocity_percentage2,
        };
        ret_package
    }
    
    fn time_difference(&self) -> f64 {
        println!("The time difference is :{}", self.after_time - self.before_time);
        self.after_time - self.before_time
    }
}

struct IdealTwoCase {
    case1_va: f64,
    case1_vb: f64,
    case1_percentage: f64,
    case2_va: f64,
    case2_vb: f64,
    case2_percentage:f64,
}

fn before_after_collision_speed (data_set: &Vec<Vec<f64>>, time: &f64, v_before_i: &u8, v_after_i:&u8) -> BeAfCollsion {
    let v_after_i: usize = *v_after_i as usize; 
    let v_before_i: usize = *v_before_i as usize; 

    let mut before_time = 0.0;
    let mut after_time = 0.0;
    let mut before_va = 0.0;
    let mut before_vb = 0.0;
    let mut after_va = 0.0;
    let mut after_vb = 0.0;

    println!("for the initial velocity  how many time interval are the most accurate to describe you initial velocity");
    
    let mut content = String::new();
    
    io::stdin().read_line(&mut content)
		.expect("failed to read the line");
		
	let mut time_stack_before: i32 = content.trim().parse().unwrap();
	
	content = String::new();
	println!("what time does it start?");
	
	io::stdin().read_line(&mut content)
		.expect("failed to read the line");
		
		
	let start_time_before: f64 = content.trim().parse().unwrap();
	
	let mut add_up_a = 0.0;
	let mut add_up_b = 0.0;
	let mut add_up_count = 0.0;
	
	let mut flag = false;
	
	for item in data_set.iter() {
		if start_time_before == item[0] || flag == true {
			flag = true;
			println!("flag:{}", flag);
			add_up_a = add_up_a + item[v_before_i].abs();
			add_up_b = add_up_b + item[v_after_i].abs();
			add_up_count += 1.0;
			time_stack_before -= 1;
			if time_stack_before == 0 {
				break;
			}
		}
	}
	
	let  before_va = add_up_a / add_up_count;
	
	let before_vb = add_up_b / add_up_count;
	
	let before_time = start_time_before;
	
	println!("the before time:{} va:{} vb:{}", before_time, before_va, before_vb);
	
	println!("for the final velocity  how many time interval are the most accurate to describe you initial velocity");
    
    let mut content = String::new();
    
    io::stdin().read_line(&mut content)
		.expect("failed to read the line");
		
	let mut time_stack_after: i32 = content.trim().parse().unwrap();
	
	println!("what time does it start?");
	
	content = String::new();
	
	io::stdin().read_line(&mut content)
		.expect("failed to read the line");
		
	let end_time_after: f64 = content.trim().parse().unwrap();
	
	let mut add_up_a = 0.0;
	let mut add_up_b = 0.0;
	let mut add_up_count = 0.0;
	
	let mut flag = false;
	
	for item in data_set.iter() {
		if end_time_after == item[0] || flag == true {
			flag = true;
			
			println!("flag:{}", flag);
			//println!("obj before_a :{}, before_b: {}", item[v_before_i], item[v_after_i]);
			add_up_a = add_up_a + item[v_before_i].abs();
			add_up_b = add_up_b + item[v_after_i].abs();
			add_up_count += 1.0;
			time_stack_after -= 1;
			if time_stack_after == 0 {
				break;
			}
		}
	}
	
	let  after_va = add_up_a / add_up_count;
	
	let after_vb = add_up_b / add_up_count;
	
	let after_time = end_time_after;
	
	println!("the after time:{} va:{} vb:{}/ add_up_count:{} add_up_a:{}, add_up_b:{}", after_time, after_va, after_vb, add_up_count, add_up_a, add_up_b);
    
    
    println!("please enter the mass which has an initial velocity");

    let ma = ask_mass();

    println!("please enter the mass which was stationary at first");

    let mb = ask_mass();

    println!("ma:{},\t mb{},\n v_t_be,{},\t v_a_be{},\t v_b_be{},\nvt_af{}, \tv_a_af{}, \tv_b_af{}", ma, mb, before_time, before_va, before_vb, after_time, after_va, after_vb);
    
    if before_time > after_time {
        panic!("the max speed of va appear after the collision not making sense");
    }

    let detailed_info = BeAfCollsion {
        ma: ma,
        mb: mb,
        before_time: before_time,
        before_va: before_va,
        before_vb: before_vb,
        after_time: after_time,
        after_va: after_va,
        after_vb: after_vb,
    };
    detailed_info

}

fn ask_mass() -> f64 {
    println!("please enter the mass. note the mass is in the unit kg");

    let mut content = String::new();

    io::stdin().read_line(&mut content)
        .expect("unable to read the line");

    let content: f64 = content.trim().parse().unwrap();
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_() {
        call_this();
    }
}
