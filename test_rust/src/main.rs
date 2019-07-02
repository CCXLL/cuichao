const MSG_ON_BLOCK:i32=1;  //作用域为整个文件

//let temp:i32 =1;    //error:let声明的局部变量只能在{}内声明使用

mod test;
mod utils;

use test::test_struct::*;
use test::test_enum::*;
use test::test_closures::*;
use test::test_generics::*;
use test::test_trait::*;

use test::*;
use std::collections::HashMap;



fn main() {
    // println!("Hello,world!");

    // let msg ="Hello,world!"; 
    // println!("msg= {}",msg);    //"!"表示调用宏不是函数 

/***************************************************************变量与可变性****************************************************/
    //let msg ;                                 //erro:无法识别类型


    // let msg:i32 ;                              //ok:


    // let msg:i32 ;                             //类型自动推导为i32  
    // msg = "hello,world!";                    //error:msg为i32类型，不能赋值str


    // let msg ;
    // msg = "hello,world!";                       //ok
    // msg = "hello,world!111";                    //error:msg为不可变变量，只能赋值一次


    // let msg ="Hello,world!"; 
    // msg = "Hello,celesos!";                  //error:let声明的变量默认为不可变


    // let mut msg ="Hello,world!"; 
    // msg = "Hello,celesos!";                //ok：可变变量使用let mut声明
    // msg = 1;                               //error:msg自动识别为str,不能赋值其它类型 

    // let msg ="Hello,world!";     
    // let msg ="Hello,celesos!";               //ok:这种方式叫做隐藏,隐藏其实是创建了一个新的变量，只是变量名是一样的，类型可以重新定义
    // let msg =1;                              //ok   


    //隐藏用法一：修改变量为可变
    // let  msg ="Hello,world!";    
    // let mut msg = msg;
    // msg="123";
    //println!("msg= {}",msg);

    //隐藏用法一：修改变量为不可变
    // let mut a = "123";
    // a="456";
    // let a=a;
    // println!("msg= {}",a);



/*******************************************************************数据类型*****************************************************************/
//整型：i8,u8,i16,u16,i32,u32,i64,u64,isize,usize
//isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。
    // let num :i8=100;
    // let num =100i8;
    // let num:isize = 100;
    // let num:usize = 100;

    
//浮点型：f32,f64
    // let num :f32=100.0;
    // let num:f64 =100.0;

//字符类型
    // let a ='a';

//bool
    // let a = true;
    // let a:bool =true; 


//数组:固定长度，类型一致
    // let array=[0,1,2,3];                       //声明一个数组，自动识别类型为整形
    // let array:[i32;5];                          //声明一个数组，类型为i32，长度为5，没有初始化，不能直接使用
    // let array=[1;20];                           //长度为20,初始值都为1
    // let len = array.len();                      //查询数组长度
    // let first = array[0];                       //获取数组第一个值

//元组:固定长度，类型随意
    // let x =(1,"a",3.0);
    // let y =(1,2,3);
    // let mut z = (4,5,6);
    // // z=x;                                           //error:类型不一致 
    // z=y;                                           //ok:类型一致，长度一致


//&str与String
//str 是被编码成UTF-8的一个字节数组
//&str:一个指向分配在某处的 String 的一部分地址，为固定容量，不可变
//String:String 是一个被拥有（owned）的在堆上分配的 UTF-8 的字节缓冲区。可变 String 可以被修改，根据需要增加其容量
    // let a="abc";                                    //&str
    // let b= String::from("abc");  
    // let a = &b[0..];                                //&str

    // let a=String::new();                            //String,空串     
    // let a= String::from("abc");                     //String
    // let a="abc".to_string();                        //String
    // let a="abc".to_owned();                         //String

    // let mut a=String::from("abc");  
    // a.push_str("d");            a[100];

    // let b="d";
    // a.push_str(b);              a[100];
    

    // let a=String::from("abc");  a[100];
    // let b = "def";
    // let c=a+&b;                                      //abcdef  使用 + 运算符时调用的方法签名的第一个参数为string，第二个为String引用，调用+后第一个参数不可用

    // let a=String::from("abc");
    // let b=String::from("def"); 
    // let e=a+&b;                                     //abcdef  &String被强转为&str
    // let d = format!("{}-{}",c,b);                   //adbdef-def 使用format链接字符串

    // let mut a=String::from("abc"); 
    // a.insert(1, 'a');                                   //aabc
    // a.remove(1);                                        // abc    
    // let b = a.to_uppercase();                           //ABC 
    // let b =b.to_lowercase();                            //abc

    

    //  let d=String::from("你好");
    //   println!("a= {}",d.len());                        //6,一个汉字占用三个字节

   
    // let mut b=String::from("abc");                     //遍历字符串    
    // for  i in b.chars(){
    //     println!("i= {}",i); 
    // }
                          

    // let a = String::from("");
    // println!("{}",a.is_empty());                        //true
    // let a = String::from("  ");
    // println!("{}",a.is_empty());                        //false
    // let a = String::new();
    // println!("{}",a.is_empty());                        //true

//slice
//slice数据类型，没有所有权。它允许引用集合中一段连续的元素序列，而不用引用整个集合
    // let a = String::from("Hello,world!");
    // let hello = &a[0..5];                               //这里是字节偏移，而不是字符偏移
    // let world = &a[6..];
    // println!("hello= {}{}",hello,world);

    // let a =String::from("女足世界杯");
    // let b = &a[0..3];                                   //女

   



 /*******************************************常量*************************************************/   
 //规则1：声明常量使用 const 关键字而不是 let，并且必须注明值的类型
    // const MSG_ON_BLOCK=1;           //error:必须声明类型
    // const MSG_ON_BLOCK：i32=1;      //ok

 
 //规则2：const声明的常量可以在任何作用域中声明，let声明的局部变量只能在{}内声明使用
     //const MSG_ON_BLOCK：f32=1;      //error:外部已定义一个  MSG_ON_BLOCK不可重复定义

 //规则：常量的初始化表达式一定要是一个编译器常量，不能是运行期值

//规则：常量绝对不能使用mut修饰，因为mut是用来声明可变局部变量的



/*******************************************结构体*************************************************/ 
    //test_struct();


/*******************************************枚举*************************************************/ 
    // test_enum();


/*******************************************vector*************************************************/   
    //vector 存储同一类型的值，长度可变
    //创建
    // let v:Vec<i32> = Vec::new();
    // let v = vec![1,2,3];                       //Vec<32>
    

    // let mut v:Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // println!("{:?}",v);                     // [1,2,3]

    // v.remove(1);                           //index
    // println!("{:?}",v);                    //[1,3]

    // v.insert(1,4);
    // println!("{:?}",v);                    //[1,4,3]

   

    // let mut v1=vec![5,6,7];
    // v.append(&mut v1); 
    // println!("{:?}",v);                     //[1,4,3,5,6,7]

    // let a:&i32=&v[1];
    // println!("{}",a);

    // v.sort();
    // println!("{:?}",v);                     //升序 [1, 3, 4, 5, 6, 7]

    
/*******************************************map*************************************************/ 
    //map 健值对，健和值都必须是同一类型
    // let mut m:HashMap<String,i32> = HashMap::new();
    // m.insert(String::from("a"), 1);
    // m.insert(String::from("b"), 2);
    // m.insert(String::from("c"), 3);

    // println!("{:?}",m);

    // let a = match m.get(&String::from("aa")){           //Option<&V>
    //     Option::Some(value)=> value,
    //     Option::None=>&-1
    // };
    // println!("{}",a);


    // for (key,value) in &m{                              //遍历map
    //     println!("key:{},value:{}",key,value);
    // }

    //  let keys = m.keys();                               //获取所有的key
    //  for key in keys{
    //      println!("key:{}",key);
    //  }


    // m.entry(String::from("aa")).or_insert(0);    //如果没有aa这个key的值，插入默认组0   

 /*******************************************泛型*************************************************/  
    // test_generics();


 /*******************************************trait*************************************************/  
    // test_trait();

   /*******************************************闭包*************************************************/   
    // test_closures();




    // let mut s = String::from("abc");

    // test_string(&mut s);
    // println!("{}",s);
}


fn test_string(s:&mut String){
    s.push_str("ff");
}