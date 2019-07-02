
pub mod test_struct{
    #[derive(Debug)]
    pub struct User{
        pub name:String,
        pub age : u32,
        pub password: String
    }
    impl User{
       pub fn createUser(name:String,age:u32,password:String)->User{                    //关联函数，和结构体相关联  User::createUser
            User{name,age,password}
        }

        pub fn createUser1(&self,name:String,age:u32,password:String)->User{            ////方法，和结构体实例相关 user1.createUser1
            User{name,age,password}
        }
    }

    struct color(i32,i32,i32); //元组结构体

    pub fn test_struct(){
        let user1 =User::createUser(String::from("aa"),18,  String::from("123456"));
        let mut user2 = User{
            name:String::from("bb"),    
            ..user1
        };

        user2.name=String::from("aaaa");
        println!("{:?}",user2);

        user2.createUser1(String::from("aa"),18,  String::from("123456"));
        


        let cl = color(1,2,3);//元组结构体
        println!("{}",cl.0);
    }
}

pub mod test_enum{
    #[derive(Debug)]
     enum Url{
        debug(i32,i32),
        release{x:i32,y:i32},
        // other
    }
    impl Url{
        fn getDebug(&self){
           
        }
    }
   pub fn test_enum(){
       let d = Url::debug(12,23);
       let r = Url::release{x:12,y:121};      

       println!("{:?}",r);

       let a = match r{
           Url::debug(x,y)=>x,
           _=>0
       };
       println!("{:?}",a);

   }
}

//泛型
pub mod test_generics{          

    fn show<T>(a:T)->T{
        a
    }

    struct Point<T>{
        x:T,
        y:T
    }

    impl<T> Point<T>{
        fn x(&self)->&T{
             &self.x
        }
    }

    enum Option<T> {
        Some(T),
        None,
    }   

    pub fn test_generics(){
        println!("{}",show(1) );
        println!("{}",show("abc") );

        let a=Point{x:1,y:2};
        println!("{}",a.x );
        println!("{}",a.x());
        //let a=Point{x:"1",y:2};

    }

}

pub mod test_trait{
    pub trait Animal{
        fn run(&self);
        fn eat(&self){
                
        }
    }

    struct Cat{
        age:i32,
        color:String
    }

    impl Animal for Cat{
        fn run(&self){
            println!("The {} cat is running!",self.color);
        }
    }

    struct Dog{
        age:i32,
        color:String
    }

    impl Animal for Dog{
        fn run(&self){
            println!("The {} dog is running!",self.color);
        }
    }


    fn test_run<T:Animal>(animal:&T){
           animal.run(); 
    }

    pub fn test_trait(){
        let cat= Cat{age:2,color:String::from("black")};
        let dog = Dog{age:2,color:String::from("black")};
        test_run(&cat);
        test_run(&dog);
    }

}

//闭包
pub mod test_closures{
    pub fn test_closures(){

//Fn：是不可变化的fn; 
// FnMut：是可变的fn； 
// FnOnce：是一次性消费的fn； 

        // let a=4;                                     //fn
        // let x = || println!("{}",a);
        // x();
        // x();

        // let mut a =5;                                //FnMut
        // let mut x =||{
        //     a+=1;
        //     println!("{}",a);
        // };

        // x();    
        // x(); 
            

    }   

}


pub mod test_iterator{

    struct testnum{
        num:i32
    }

    impl Iterator for testnum{

        type Item  =i32;
        fn next(&mut self)->Option<Self::Item>{
            None
        }
         
    }
  
}



pub fn show(){
    println!("this is function!");
}