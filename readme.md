do
    v <- getSensor(0)
    u <- getSensor(1)
    let x = (v + 25) * 100 + u;
    return x

getSensor(0).bind(|v| {getSensor(1).bind(|u| {let x = (v + 25) * 100 + u; Maybe::<usize>::ret(x)})})

a >>= b = a.bind(&|&x| b(a));

t(s : as) = s : t(as)
t(s.? : as) = s.bind(&|&a| t(as))
t([] = return ()) 