#[macro_use] extern crate prettytable;
use prettytable::Table;

#[derive(Copy, Clone)]
enum D{
	Mon = 0,
	Tue = 1,
	Wed = 2,
	Thu = 3,
	Fri = 4
}
#[derive(Clone)]
struct Subject{
	name:	String,
	sched:	Vec<(D, usize, usize)>,
	sched_l:Vec<(D, usize, usize)>
}


fn check_conflict(class: &(D, usize, usize), timetable: &[[String; 11]; 5]) -> bool{
	for i in class.1 ..= class.2 {
		if timetable[class.0 as usize][i-9] != "" {
			return true;
		}
	}
	return false;
}

fn add_class(class: &(D, usize, usize), name: String, timetable: &mut [[String; 11]; 5]){
	for i in class.1 ..= class.2 {
		timetable[class.0 as usize][i-9] = name.clone();
	}
}

fn remember(timetable: Option<[[String; 11]; 5]>) -> Vec<[[String; 11]; 5]>{
	static mut ALL_TIMETABLES: Vec<[[String; 11]; 5]> = Vec::new();
	unsafe {
		if timetable.is_some() {
			ALL_TIMETABLES.push(timetable.unwrap());
		}
		return ALL_TIMETABLES.clone();
	}
}

fn generate_perm(subjects: Vec<Subject>, timetable: [[String; 11]; 5]){
	if subjects.len() == 1{
		for each_sched in &subjects[0].sched{
			if check_conflict(&each_sched, &timetable){
				continue;
			}
			let mut temptable = timetable.clone();
			add_class(each_sched, subjects[0].name.clone(), &mut temptable);
			for each_lab in &subjects[0].sched_l{
				if check_conflict(&each_lab, &temptable) {
					continue;
				}
				let mut temp2ble = temptable.clone();
				add_class(each_lab, subjects[0].name.clone() + " lab", &mut temp2ble);
	
				remember(Some(temp2ble));
			}
		}
		return;
	}
	for each_sched in &subjects[0].sched{
		if check_conflict(&each_sched, &timetable){
			continue;
		}
		let mut temptable = timetable.clone();
		add_class(each_sched, subjects[0].name.clone(), &mut temptable);
		for each_lab in &subjects[0].sched_l{
			if check_conflict(&each_lab, &temptable) {
				continue;
			}
			let mut temp2ble = temptable.clone();
			add_class(each_lab, subjects[0].name.clone() + " lab", &mut temp2ble);

			let mut subjects_minus = subjects.clone();
			subjects_minus.remove(0);
			//early quit if no class or lab can be added to timetable?
			generate_perm(subjects_minus, temp2ble.clone());
		}
	}
}

fn print_table(timetable: &[[String; 11]; 5]){
	let mut table = Table::new();
	table.add_row(row![9,10,11,12,13,14,15,16,17,18,19]);
	for day in timetable {
		let mut vec: Vec<prettytable::Cell> = vec![];
		for class in day {
			vec.push(prettytable::Cell::new(class).style_spec("bFg"));
		}
		table.add_row(prettytable::Row::new(vec));
	}
	table.printstd();
}
fn main() {
	let subjects = Vec::from([  //todo: make this shit less tedious to do
	Subject{
		name:	"Calc 2".to_string(),
		sched:	[(D::Mon, 12,14), (D::Mon, 15,17)].to_vec(),
		sched_l:[(D::Mon, 15,16), (D::Tue, 14,15), (D::Thu, 9,10)].to_vec()
	},
    Subject{
		name:	"DBI".to_string(),
		sched:	[(D::Mon, 9,10), (D::Mon, 11,12)].to_vec(),
		sched_l:[(D::Tue, 9,10), (D::Tue, 11,12), (D::Tue, 13,14), (D::Wed, 9,10),
				 (D::Thu, 9,10), (D::Thu, 11,12), (D::Thu, 17,18)].to_vec(),
	},
	Subject{
		name: 	"Prog 2".to_string(),
		sched:	[(D::Wed, 17,19), (D::Fri, 17,19)].to_vec(),
		sched_l:[(D::Tue, 9,10), (D::Thu, 9,10), (D::Thu, 11,12), (D::Thu, 13,14),
				 (D::Fri, 9,10)].to_vec()
	}]);

	let timetable: [[String; 11]; 5] = Default::default();
	generate_perm(subjects, timetable);
	
	let allofthem = remember(None);

	let empty_day: [String; 11] = Default::default();
	let mut besttables: Vec<[[String; 11] ;5]> = vec![];
	let mut bestsize: usize = 9999;
	for each in &allofthem { //find the one with smallest time
		let mut length = 0;
		for day in each {
			if day == &empty_day {
				length -= 1; //add incentive to prefer empty days
				continue;
			};
			let mut begin = 0;
			let mut end = 10;
			while day[begin] == "" || day[end] == "" {
				if day[begin] == "" {begin += 1;};
				if day[end] == "" {end -= 1;};
			}
			length += end - begin + 1;
		}
		if length < bestsize {
			besttables.clear();
			besttables.push(each.clone());
			bestsize = length;
		} else if length == bestsize {
			besttables.push(each.clone());
		}
	}

	for each in besttables {
		println!("{}", bestsize);
		print_table(&each);
	}
}
