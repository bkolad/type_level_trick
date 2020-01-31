use std::marker::PhantomData;

struct MIMCType{}
struct POSSEIDONType{}

struct Native<'a, T: 'a>{
    phantom: PhantomData<&'a T>,
}

struct Gadget<'a, T: 'a>{
    phantom: PhantomData<&'a T>,
}



fn main() {
    type Hash = MIMCType;

    let gadget= Gadget::<Hash>{
        phantom: PhantomData,
    };

    let native= Native::<Hash>{
        phantom: PhantomData,
    };


    force_the_same_hash(gadget, native);

    //If we pass Native, Gadget with diffrent types it wont compile
    let gadget1= Gadget::<POSSEIDONType>{
        phantom: PhantomData,
    };

    let native1= Native::<Hash>{
        phantom: PhantomData,
    };
    force_the_same_hash(gadget, native);

}


fn force_the_same_hash<T>(g:Gadget::<T>, native:Native<T>){

}
