impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if (sx, sy) == (fx, fy) && t == 1{
            return false;
        }
        i32::max((fx - sx).abs(), (fy - sy).abs()) <= t
    }
}