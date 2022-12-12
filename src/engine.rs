use std::{
    io::{self, Read, Stdout, Write},
    sync::mpsc::{channel, Receiver},
    thread,
};

use rand::Rng;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::{apple::Apple, board::Board, position::Position, snake::Snake};

pub fn run() {
    let input_channel = create_input_channel();
    let mut screen = create_rendering_environment();

    let board = Board::new(80, 20);
    let mut snake = Snake::new(&board);
    let mut apple = Apple::new(get_random_apple_position(&snake, &board));

    loop {
        if let Ok(input) = input_channel.try_recv() {
            if let 'q' = input {
                print!("User quit requested, game over! ");
                break;
            } else {
                snake.process_input(input)
            }
        }

        snake.tick();

        if snake.head() == &apple.position {
            snake.consume_apple(apple);
            apple = Apple::new(get_random_apple_position(&snake, &board));
        }

        if snake
            .body
            .iter()
            .rev()
            .take(snake.len as usize)
            .skip(1)
            .any(|pos| pos == snake.head())
        {
            print!("Snake ate itself, game over! ");
            break;
        }

        if snake.head().x < 0
            || snake.head().x >= board.width as i8
            || snake.head().y < 0
            || snake.head().y >= board.height as i8
        {
            print!("Snake hit stage bounds, game over! ");
            break;
        }

        render(&mut screen, &snake, &board, &apple);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    print!("Press any key to continue.");
    screen.flush().unwrap();
    input_channel.recv().unwrap();
    cleanup_rendering_environment(&mut screen);
}

fn create_input_channel() -> Receiver<char> {
    let (tx, rx) = channel::<char>();
    thread::spawn(move || loop {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).unwrap();
        tx.send(char::from_u32(buffer[0] as u32).unwrap()).unwrap();
    });
    rx
}

fn create_rendering_environment() -> RawTerminal<Stdout> {
    let mut screen = io::stdout().into_raw_mode().unwrap();
    write!(screen, "{}", termion::screen::ToAlternateScreen).unwrap();
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    screen
}

fn cleanup_rendering_environment(screen: &mut RawTerminal<Stdout>) {
    write!(screen, "{}", termion::screen::ToMainScreen).unwrap();
}

fn get_random_apple_position(snake: &Snake, board: &Board) -> Position {
    let possible_positions = (0..board.width as i8)
        .flat_map(|x| (0..board.height as i8).map(move |y| Position::new(x, y)))
        .filter(|pos| {
            !snake
                .body
                .iter()
                .rev()
                .take(snake.len as usize)
                .any(|seg| pos == seg)
        })
        .collect::<Vec<Position>>();
    let index = rand::thread_rng().gen_range(0..possible_positions.len());
    possible_positions[index]
}

fn render(screen: &mut RawTerminal<Stdout>, snake: &Snake, board: &Board, apple: &Apple) {
    (-1..=board.height as i8).for_each(|y| {
        write!(screen, "{}", termion::cursor::Goto(1, (y + 2) as u16)).unwrap();
        (-1..=board.width as i8).for_each(|x| {
            let pos = Position::new(x, board.height as i8 - (y + 1));
            if pos == apple.position {
                write!(screen, "b").unwrap();
            } else if snake.head() == &pos {
                write!(screen, "@").unwrap();
            } else if snake
                .body
                .iter()
                .rev()
                .take(snake.len as usize)
                .any(|seg| seg == &pos)
            {
                write!(screen, "o").unwrap();
            } else if pos.x == -1
                || pos.x == board.width as i8
                || pos.y == -1
                || pos.y == board.height as i8
            {
                write!(screen, "*").unwrap()
            } else {
                write!(screen, " ").unwrap();
            }
        })
    });

    write!(
        screen,
        "{}[ Snake ] w/a/s/d: Move | q: Quit",
        termion::cursor::Goto(1, (board.height + 3) as u16)
    )
    .unwrap();

    write!(
        screen,
        "{}",
        termion::cursor::Goto(1, (board.height + 4) as u16)
    )
    .unwrap();

    screen.flush().unwrap();
}
