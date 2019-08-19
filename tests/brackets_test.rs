use rpds::Stack;

fn is_brackets_corrects(brackets: String) -> bool {
    let mut stack: Stack<char> = Stack::new();
    let chars:Vec<char> = brackets.clone().chars().collect();
    let mut result = true;
    for mut index in 0..brackets.len(){
        if chars[index] =='{'{
            stack=stack.push('{');
        }else{
            match stack.peek() {
                None => {
                    stack.pop_mut();
                    println!("error on {}. Need {{ before.",index);
                    index = brackets.len();
                    result=false;
                },
                Some(value) => {
                    match value {
                        '}' => {
                            println!("error on {}. Need {{ before.",index);
                            index = brackets.len();
                            result=false;
                        },
                        '{' => {
                            stack.pop_mut();
                        },
                        _ => unreachable!()
                    }
                }
            }
        }
    }
    if !stack.is_empty(){
        result=false;
        println!("error on {}, Need closed bracket", brackets.len())
    }
    result
}

#[test]
fn is_brackets_corrects_test(){
    assert_eq!(true,is_brackets_corrects("{}{}{{}}".to_string()));
    assert_eq!(true,is_brackets_corrects("{{}}".to_string()));
    assert_eq!(true,is_brackets_corrects("{{}{}}{{{}}}".to_string()));
    assert_eq!(false,is_brackets_corrects("}{}".to_string()));
    assert_eq!(false,is_brackets_corrects("{}{".to_string()));

}