fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let vv = &v[1..3];
    let mut vvc = vv.to_vec();
    let mut vvcc: Vec<i32> = vv.into();
    vvc.push(100);
    vvcc.push(200);

    println!("{:?}", v);   // 参照されただけなのでvの束縛は健在
    println!("{:?}", vv);   // to_vecでは新規にVecを作るのでvvの束縛は健在
    println!("{:?}", vvc);
    println!("{:?}", vvcc)
}