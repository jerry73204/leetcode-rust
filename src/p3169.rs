//! # 3169. Count Days Without Meetings
//! [Submission](https://leetcode.com/problems/count-days-without-meetings/submissions/1584536664)

use crate::Solution;

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|meeting| meeting[0]);

        let mut count = 0;
        let mut last_end = 1;

        for meeting in meetings {
            let &[start, end] = meeting.as_slice() else {
                unreachable!()
            };
            count += (start - last_end).max(0);
            last_end = last_end.max(end + 1);
        }

        count += days + 1 - last_end;

        count
    }
}
