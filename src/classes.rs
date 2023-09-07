use crate::{Subject, D};

pub fn empty_day_incentive() -> usize {
	return 2;	//the larger the number, the bettr the algorithm will rank timetables with empty days.
}	//1 and 2 work really well, put 0 to disable empty day prioritization.

pub fn get_subjects() -> Vec<Subject> {
	return Vec::from([  //todo: make this shit less tedious to do
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
		},
		Subject{
			name:	"Phys 2".to_string(),
			sched:	[(D::Wed, 15,16)].to_vec(),
			sched_l:[(D::Wed, 17,18), (D::Thu, 17,18)].to_vec()
		},
		Subject{
			name:	"SR&W".to_string(),
			sched:	[(D::Tue, 14,15)].to_vec(),
			sched_l:[(D::Tue, 16,16)].to_vec()
		}
	]);
}