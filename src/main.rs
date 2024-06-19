fn total(win: i32, loss: i32) -> i32 {
    win - loss
}

fn rank(n: i32) -> &'static str {
    match n {
        n if n <= 10 => "Ferro",
        n if n <= 20 => "Bronze",
        n if n <= 50 => "Prata",
        n if n <= 80 => "Ouro",
        n if n <= 90 => "Diamante",
        n if n <= 100 =>"Lendário",
        _ => "Imortal"
    }
}

fn main() {
    let total = total(99, 42);
    let rank = rank(total);

    println!("O Herói tem um saldo de {total} e está no nível de {rank}");
}
