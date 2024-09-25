# Phoenix Engine

Phoenix är en simpel spelmotor skriven i programspråket Rust. Motorn nyttjar OpenGL bindningar för dess `rastrering` och Slint språket för dess gränsnitt. För oss är Phoenix ett perfekt projekt för att finslipa våra utvecklingskunskaper. Spelmotorn blir vårat gymnasiearbete på NTI Gymnasiet i Sundsvall 2024.

Under projektets gång kommer vi att lära oss mer om:

1. Programspråket Rust.
2. Grafiks programmering med OpenGL och Slint.
3. Korrekt användning av GitHub issues, pull requests, merge requests och commits.

:copyright: [Neo Mannskär](https://github.com/neomannskar) & [Pontus Henriksson](https://github.com/pontushenriksson)

## Resurser

### Fysiska resurser

* [Elementary Linear Algebra 10th Edition, with supplemental applications (Howard Anton, Chris Rorres)](https://books.google.se/books/about/Elementary_Linear_Algebra_with_Supplemen.html?id=I8GNPgAACAAJ&redir_esc=y)

### Digitala resurser

* [The Rust Book](https://doc.rust-lang.org/book/)
* [Learn OpenGL](https://learnopengl.com/Getting-started/OpenGL)
* [Slint](https://releases.slint.dev/)
* [Proper Github Etiquette](https://betterprogramming.pub/git-workflow-etiquette-f22d96b8b0b8)

#### Videor

* [En bra serie som täcker grunderna och principerna av att skapa en spelmotor (TheCherno)](https://youtube.com/playlist?list=PLlrATfBNZ98dC-V-N3m0Go4deliWHPFwT&si=VCfmACkznrGt7yWt)
* [Fantastisk genomgång av 'The Rust Book' (Let's Get Rusty)](https://youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&si=6F9_wdnwF-yI-e8B)
* [Underskattad resurs för OpenGL programmering i C++ (VictorGordan)](https://www.youtube.com/@VictorGordan/)
* [Grundläggande genomgång i att skapa en spelmotor i Rust och OpenGL (logaMaster)](https://www.youtube.com/playlist?list=PL6TfJEvHZ7C--kM59vKUwNnh30ngWZKUD)

## Funktioner

### Planerade funktioner

* Rendering av statiska objekt
* Simpelt användargränssnitt
* Rendering av dynamiska objekt
* Simpel fysikmotor

### Eventuella funktioner

* Node-system för entiteter
* Skriptmotor med [The Lotus Programming Language](https://github.com/totem-studios/lotus)
* Animationsmotor

## Projektstruktur

phoenix_engine/
├── phoenix_core/           # The core engine as a library
│   ├── src/
│   │   ├── lib.rs          # All rendering, fysik, input, mm.
│   └── Cargo.toml          # Definerar kärnan som ett Rust library (crate)
├── phoenix/                # Applikationen (standalone application)
│   ├── src/
│   │   ├── main.rs         # Phoenix som applikation
│   └── Cargo.toml          # Definerar applikationen som en binary crate
└── phoenix_projects/       # Mapp för projekt gjorda med phoenix
    ├── phoenix-test-game/
    │   ├── src/
    │   │   ├── script.rs   # Spel logik (spelprojekt som använder phoenix_core som ett bibliotek)
    │   └── Cargo.toml      # Separat spelprojekt, phoenix_core som en dependency
