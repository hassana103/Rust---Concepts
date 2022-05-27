fn main() {

    //define variable
    let mut x = 5;
    println!("the value of x: {}", x);

    x=6;
    println!("the value of x: {}", x);

    //constance variable
    //const variables should be type anotated
    //const variables cant be assaigned by return value of a method
    //const variables cant be mut
    const Y:u32 = 1000;
    println!("the value of y: {}", Y);


    //shadowing
    //declare new variable with same name

    //data type

        //scaler data type
            //int
            //float
            //boolean
            //char

        //compand data type
            //1.tuple -> fixed size array of related data with differet types
            // define tuple
            let tup = ("value1" , 2);

                //1.1.getting value from tuple
                    //a. destructure
                    let (a,b)=tup;
                    println!("a: {0} and b: {1}",a,b);

                    //b. don notation
                    let v1=tup.0;
                    let v2=tup.1;
                    println!("v1: {0} and v2: {1}" , v1,v2);

            //2. Array -> fixed size
            //deine array
            let arr1 = [100 , 200 , 300];

                //2.1. getting value
                let v3 = arr1[2];
                println!("v3: {}" , v3);

    my_method(32 , 52);

    let sum = my_method2(10 , 2);
    println!("sum: {}" , sum);


    //control flow
        //1. if
        let num1 = 10;

        if num1 < 5 {
            println!("if");
        }else if num1 > 12 {
            println!("else if");
        }else {
            println!("else")
        }
        // if in let
        let condition = true;
        let num2 = if condition {5} else {6};

        println!("{}",num2);

        //2. loops

            //2.1 loop -> goes forever or break
            loop {
                println!("in loop");
                break
            }

                //2.1.1 loop can return value
                let mut counter = 0;
                let res = loop {
                    counter +=1;
                    if counter >= 10 {
                        break counter;
                    }
                };
                println!("{}" , res);


            //2.2 while loop
                let mut counter = 10;
                while counter >0 {
                    counter -=1;
                    println!("{}" , counter);
                }

            //2.3 for loop
                //foreach
                let arr2 = [100 , 200 , 300 , 400 , 500];
                for element in arr2.iter(){

                    println!("{}" , element);
                }

                //classic for in range
                for num3 in 1..5{ // range [1...5) "last number is exclusive"
                    println!("{}" , num3);
                }




}


//methods
fn my_method(x: i32 , y:i32){
    println!("my other method {0} , {1}", x,y)
}

//method with return type
fn my_method2(a:i32 , b:i32) -> i32{
    return a+b;
}
