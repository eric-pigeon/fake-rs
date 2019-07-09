use crate::faker::*;
use crate::Fake;

pub struct Faker;

impl Fake for Faker {
    #[inline(always)]
    fn name_first_name_data() -> &'static [&'static str] {
        data::NAME_FIRST_NAME
    }

    #[inline(always)]
    fn name_last_name_data() -> &'static [&'static str] {
        data::NAME_LAST_NAME
    }
}
impl Address for Faker {}
impl Boolean for Faker {}
#[cfg(feature = "chrono")]
impl Chrono for Faker {}
impl Company for Faker {}
impl Internet for Faker {}
#[cfg(feature = "http")]
impl Http for Faker {}
impl Lorem for Faker {}
impl Name for Faker {
    #[inline]
    fn name() -> String {
        format!("{}{}", Self::last_name(), Self::first_name())
    }

    #[inline]
    fn name_with_middle() -> String {
        format!("{}{}", Self::last_name(), Self::first_name())
    }
}
impl Number for Faker {}
impl PhoneNumber for Faker {}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod data {
    pub static NAME_FIRST_NAME: &'static [&'static str] = &["怡君", "欣怡", "雅雯", "心怡", "志豪", "雅婷", "雅惠", "家豪", "雅玲", "靜怡", "志偉", "俊宏", "建宏", "佩君", "怡婷", "淑芬", "靜宜", "俊傑", "怡如", "家銘", "佳玲", "慧君", "怡伶", "雅芳", "宗翰", "志宏", "淑娟", "信宏", "志強", "淑婷", "佩珊", "佳慧", "佳蓉", "佳穎", "淑惠", "智偉", "欣儀", "嘉玲", "雅慧", "惠雯", "玉婷", "惠如", "惠君", "宜芳", "惠婷", "淑華", "志明", "雅芬", "家榮", "俊賢", "俊豪", "慧玲", "嘉宏", "佩芬", "佳樺", "雅琪", "淑萍", "淑君", "婉婷", "佳琪", "韻如", "詩婷", "建良", "芳儀", "宜君", "佩蓉", "志銘", "雅鈴", "建文", "佩玲", "鈺婷", "雅萍", "立偉", "文傑", "慧如", "淑慧", "佳宏", "志遠", "靜儀", "惠玲", "淑玲", "美君", "怡慧", "千慧", "馨儀", "嘉慧", "家瑋", "美慧", "美玲", "建志", "宗憲", "筱婷", "靜雯", "雅君", "彥廷", "怡靜", "玉玲", "郁婷", "俊男"];
    pub static NAME_LAST_NAME: &'static [&'static str] = &["趙", "錢", "孫", "李", "周", "吳", "鄭", "王", "馮", "陳", "褚", "衛", "蔣", "沈", "韓", "楊", "朱", "秦", "尤", "許", "何", "呂", "施", "張", "孔", "曹", "嚴", "華", "金", "魏", "陶", "薑", "戚", "謝", "鄒", "喻", "柏", "水", "竇", "章", "雲", "蘇", "潘", "葛", "奚", "範", "彭", "郎", "魯", "韋", "昌", "馬", "苗", "鳳", "花", "方", "俞", "任", "袁", "柳", "酆", "鮑", "史", "唐", "費", "廉", "岑", "薛", "雷", "賀", "倪", "湯", "滕", "殷", "羅", "畢", "郝", "鄔", "安", "常", "樂", "於", "時", "傅", "皮", "卞", "齊", "康", "伍", "餘", "元", "蔔", "顧", "孟", "平", "黃", "和", "穆", "蕭", "尹", "姚", "邵", "湛", "汪", "祁", "毛", "禹", "狄", "米", "貝", "明", "臧", "計", "伏", "成", "戴", "談", "宋", "茅", "龐", "熊", "紀", "舒", "屈", "項", "祝", "董", "梁", "杜", "阮", "藍", "閔", "席", "季", "麻", "強", "賈", "路", "婁", "危", "江", "童", "顏", "郭", "梅", "盛", "林", "刁", "鍾", "徐", "邱", "駱", "高", "夏", "蔡", "田", "樊", "胡", "淩", "霍", "虞", "萬", "支", "柯", "昝", "管", "盧", "莫", "柯", "房", "裘", "繆", "幹", "解", "應", "宗", "丁", "宣", "賁", "鄧", "鬱", "單", "杭", "洪", "包", "諸", "左", "石", "崔", "吉", "鈕", "龔", "程", "嵇", "邢", "滑", "裴", "陸", "榮", "翁", "荀", "羊", "于", "惠", "甄", "曲", "家", "封", "芮", "羿", "儲", "靳", "汲", "邴", "糜", "松", "井", "段", "富", "巫", "烏", "焦", "巴", "弓", "牧", "隗", "山", "穀", "車", "侯", "宓", "蓬", "全", "郗", "班", "仰", "秋", "仲", "伊", "宮", "甯", "仇", "欒", "暴", "甘", "鈄", "曆", "戎", "祖", "武", "符", "劉", "景", "詹", "束", "龍", "葉", "幸", "司", "韶", "郜", "黎", "薊", "溥", "印", "宿", "白", "懷", "蒲", "邰", "從", "鄂", "索", "鹹", "籍", "賴", "卓", "藺", "屠", "蒙", "池", "喬", "陽", "鬱", "胥", "能", "蒼", "雙", "聞", "莘", "党", "翟", "譚", "貢", "勞", "逄", "姬", "申", "扶", "堵", "冉", "宰", "酈", "雍", "卻", "璩", "桑", "桂", "濮", "牛", "壽", "通", "邊", "扈", "燕", "冀", "浦", "尚", "農", "溫", "別", "莊", "晏", "柴", "瞿", "閻", "充", "慕", "連", "茹", "習", "宦", "艾", "魚", "容", "向", "古", "易", "慎", "戈", "廖", "庾", "終", "暨", "居", "衡", "步", "都", "耿", "滿", "弘", "匡", "國", "文", "寇", "廣", "祿", "闕", "東", "歐", "殳", "沃", "利", "蔚", "越", "夔", "隆", "師", "鞏", "厙", "聶", "晁", "勾", "敖", "融", "冷", "訾", "辛", "闞", "那", "簡", "饒", "空", "曾", "毋", "沙", "乜", "養", "鞠", "須", "豐", "巢", "關", "蒯", "相", "查", "後", "荊", "紅", "遊", "竺", "權", "逮", "盍", "益", "桓", "公", "萬俟", "司馬", "上官", "歐陽", "夏侯", "諸葛", "聞人", "東方", "赫連", "皇甫", "尉遲", "公羊", "澹台", "公冶", "宗政", "濮陽", "淳于", "單於", "太叔", "申屠", "公孫", "仲孫", "軒轅", "令狐", "徐離", "宇文", "長孫", "慕容", "司徒", "司空"];
}
