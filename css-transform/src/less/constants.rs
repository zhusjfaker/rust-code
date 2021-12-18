pub struct LessMath {
    ALWAYS: i32,
    PARENS_DIVISION: i32,
    PARENS: i32,
}


pub const Math: LessMath = LessMath {
    ALWAYS: 0,
    PARENS_DIVISION: 1,
    PARENS: 2,
};


pub struct LessRewriteUrls {
    OFF: i32,
    LOCAL: i32,
    ALL: i32,
}

pub const RewriteUrls: LessRewriteUrls = LessRewriteUrls {
    OFF: 0,
    LOCAL: 1,
    ALL: 2,
};