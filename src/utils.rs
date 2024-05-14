/// 计算两个色值的差异,返回值范围:[0,100]
/// 当输入的两个色值完全相同时，返回值为0
/// 当输入的两个色值完全不同时，返回值为100
pub fn calc_color_distance(color1: (u8,u8,u8), color2: (u8,u8,u8)) -> f32{
    const SQRT_3: f32 = 1.7320508075688772; // 直接定义sqrt(3)

    // 将RGB值从[0, 255]转换为[0, 1]
    let color1_normalized = (
        color1.0 as f32 / 255.0,
        color1.1 as f32 / 255.0,
        color1.2 as f32 / 255.0,
    );
    let color2_normalized = (
        color2.0 as f32 / 255.0,
        color2.1 as f32 / 255.0,
        color2.2 as f32 / 255.0,
    );

    // 分别计算R、G、B三个通道的差值的平方
    let diff_r = (color1_normalized.0 - color2_normalized.0).powi(2);
    let diff_g = (color1_normalized.1 - color2_normalized.1).powi(2);
    let diff_b = (color1_normalized.2 - color2_normalized.2).powi(2);
    
    // 欧氏距离的平方根，然后归一化到[0, 1]区间
    ((diff_r + diff_g + diff_b).sqrt() / SQRT_3).clamp(0.0, 1.0) * 100.0
}

// test
#[cfg(test)]
#[test]
fn test_calculate_color_diff(){
    let color1 = (255, 255, 255);
    let color2 = (0, 0, 0);
    assert_eq!(calc_color_distance(color1, color2), 100.0);
    assert_eq!(calc_color_distance(color1, color1), 0.0);

}