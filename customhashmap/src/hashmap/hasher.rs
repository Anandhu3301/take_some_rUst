
trait Hashtrait<K,V> {
    fn new(param1: i32) -> self;
    fn size(&self) -> i32;
    fn isEmpty(&self) -> bool;
    fn containsKey(&self,key : K) -> bool;
    fn containsValue(&self, key : K) -> bool;
    fn get(&self, key : K) -> V;
    fn put(&self, value : V );
    fn remove(&self, key : K);
}

struct HashFunction {
    size : i32
}

impl HashFunction {
    fn new(param1 : )
}

impl Hashtrait<&str,str> for HashFunction {
    const DEFAULT_CAPACITY : i8  = 4;
    const INCREASE_FACTOR : i8  = 2;
    const DEFAULTY_LOAD_FACTOR : f32 = 0.75;
    
    fn new(param1 : i32) -> self {
        HashFunction {
            size : param1
        }
    }
    fn custom_Hash_MapImpl() {

    }

}