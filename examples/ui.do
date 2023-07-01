



Main {
    children: Window {
        title: "Testi Ikkuna"
        children: [
            Box {
                onClick: () => {
                    info("Hello world")
                }
                children: [
                    Text {
                        title: "qwerty"
                    }
                ]
            }
        ]
    }
}


render()