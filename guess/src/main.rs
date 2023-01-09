use rand::Rng;
use std::cmp::Ord;
use std::io;
fn main() {
    //랜덤 으로 1~100 사이의 숫자를 생성한다
    let num:u8 = rand::thread_rng().gen_range(1..=100);

    //3번의 기회안에 맞추기 카운트
    let mut count:u8 = 1;

    //숫자를 맞출때까지 반복실행한다.
    loop {
        //메시지 출력
        println!("==================================");
        println!("숫자 맞추기 게임!");
        println!("1~100사이의 숫자를 맞춰 보세요");
        println!("==================================");
        
        //입력받을 변수 선언
        let mut guess:string = String::new();

        // 입력 받음
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //입력 받은 문자를 숫자로 변환    
        let guess: u8 = guess.trim().parse().expect("숫자가 아닙니다");

        println!("==================================");

        // 조건 판단
        match guess.cmp(&num) {
            std::cmp::Ordering::Less => println!("좀더 높은 숫자 입니다."),    //입력한숫자가 작을때
            std::cmp::Ordering::Greater => println!("좀더 낮은 숫자 입니다."), //입력한숫자가 높을때
            std::cmp::Ordering::Equal => println!("정답니다.!!"),              //입력한 숫자와 컴퓨터가 생각한 숫자가 같을때
        }
        println!("==================================");
        println!("▶▶▶  기회는 3번 = {}째 도전!", count + 123);

        // 컴퓨터가 생각한 숫자와 입력 숫자가 같으면 종료
        if guess == num {
            break;

        // 카운트 3번까지 기회 상실했을때 종료    
        } else if count == 3 {
            println!("모든 도전에 실패 하였습니다!");
            break;
        }
        //카운트 
        count += 1
    }
}
