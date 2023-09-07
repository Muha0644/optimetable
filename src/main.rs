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
	sched_l:	Vec<(D, usize, usize)>
}


fn check_conflict(class: &(D, usize, usize), timetable: &[[String; 20]; 5]) -> bool{
	for i in class.1 ..= class.2 {
		if timetable[class.0 as usize][i] != "" {
			return true;
		}
	}
	return false;
}

fn add_class(class: &(D, usize, usize), name: &String, timetable: &mut [[String; 20]; 5]){
	for i in class.1 ..= class.2 {
		timetable[class.0 as usize][i] = name.clone();
	}
}

fn remember(timetable: Option<[[String; 20]; 5]>) -> Vec<[[String; 20]; 5]>{
	static mut ALL_TIMETABLES: Vec<[[String; 20]; 5]> = Vec::new();
	unsafe {
		if timetable.is_some() {
			ALL_TIMETABLES.push(timetable.unwrap());
		}
		return ALL_TIMETABLES.clone();
	}
}

fn solve(subjects: Vec<Subject>, timetable: [[String; 20]; 5]){
	if subjects.len() == 1{
		for each_sched in &subjects[0].sched{
			if check_conflict(&each_sched, &timetable){
				continue;
			}
			let mut temptable = timetable.clone();
			add_class(each_sched, &subjects[0].name, &mut temptable);
			for each_lab in &subjects[0].sched_l{
				if check_conflict(&each_lab, &temptable) {
					continue;
				}
				let mut temp2ble = temptable.clone();
				add_class(each_lab, &subjects[0].name, &mut temp2ble);
	
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
		add_class(each_sched, &subjects[0].name, &mut temptable);
		for each_lab in &subjects[0].sched_l{
			if check_conflict(&each_lab, &temptable) {
				continue;
			}
			let mut temp2ble = temptable.clone();
			add_class(each_lab, &subjects[0].name, &mut temp2ble);

			let mut subjects_minus = subjects.clone();
			subjects_minus.remove(0);
			//early quit if no class or lab can be added to timetable?
			solve(subjects_minus, temp2ble.clone());
		}
	}
}

fn main() {
	let subjects = Vec::from([ 
	Subject{
		name:	"Calculus 2".to_string(),
		sched:	[(D::Mon, 12,14), (D::Mon, 15,17)].to_vec(),
		sched_l:[(D::Mon, 15,16), (D::Tue, 14,15), (D::Thu, 9,10)].to_vec()
	},
    Subject{
		name:	"Database Design and Implementation".to_string(),
		sched:	[(D::Mon, 9,10), (D::Mon, 11,12)].to_vec(),
		sched_l:[(D::Tue, 9,10), (D::Tue, 11,12), (D::Tue, 13,14), (D::Wed, 9,10),
				 (D::Thu, 9,10), (D::Thu, 11,12), (D::Thu, 17,18)].to_vec(),
	},
	Subject{
		name: 	"Programming 2".to_string(),
		sched:	[(D::Wed, 17,19), (D::Fri, 17,19)].to_vec(),
		sched_l:[(D::Tue, 9,10), (D::Thu, 9,10), (D::Thu, 11,12), (D::Thu, 13,14),
				 (D::Fri, 9,10)].to_vec()
	}]);

	let timetable: [[String; 20]; 5] = Default::default();
	solve(subjects, timetable);
	
	print!("done!");
}
