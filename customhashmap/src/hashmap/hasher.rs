
trait Hashtrait<K,V> {
    fn size(&self) -> i32;
    fn isEmpty(&self) -> bool;
    fn containsKey(&self,key : K) -> bool;
    fn containsValue(&self, key : K) -> bool;
    fn get(&self, key : K) -> V;
    fn put(&self, value : V );
    fn remove(&self, key : K);

}