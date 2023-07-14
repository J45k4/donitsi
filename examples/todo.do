
PlusShape = Shape {
    vertices: [
        Vertex { x: -0.6, y: 0.1, color: "black" },
        Vertex { x: -0.1, y: 0.1, color: "black" },
        Vertex { x: -0.1, y: 0.6, color: "black" },
        Vertex { x: 0.1, y: 0.6, color: "black" }
        Vertex { x: 0.1, y: 0.1, color: "black" },
        Vertex { x: 0.6, y: 0.1, color: "black" },
        Vertex { x: 0.6, y: -0.1, color: "black" },
        Vertex { x: 0.1, y: -0.1, color: "black" },
        Vertex { x: 0.1, y: -0.6, color: "black" },
        Vertex { x: -0.1, y: -0.6, color: "black" },
        Vertex { x: -0.1, y: -0.1, color: "black" },
        Vertex { x: -0.6, y: -0.1, color: "black" }
    ]
}

MinusShape = Shape {
    vertices: [
        Vertex { x: -0.6, y: 0.1, color: "black" },
        Vertex { x: 0.6, y: 0.1, color: "black" },
        Vertex { x: 0.6, y: -0.1, color: "black" },
        Vertex { x: -0.6, y: -0.1, color: "black" }
    ]
}

todos = []

Window {
    title: "Testi Ikkuna"
    children: [
        Div {
            children: () => {
                new_todo_name = ""

                return [
                    Text {
                        text: "Todo App"
                    }
                    Div {
                        flex_direction: "Row"
                        children: [
                            TextInput {
                                placeholder: "New todo name",
                                bind_value: new_todo_name
                            },
                            Div {
                                children: [
                                    PlusShape
                                ],
                                on_click: () => {
                                    todos.push({
                                        name: new_todo_name
                                    })
                                    new_todo_name = ""
                                }
                            }
                        ]
                    }
                ]
            }
        },
        Div {
            children: todos.map(todo => {
                Div {
                    children: [
                        Text {
                            text: todo.name
                        }
                        Div {
                            children: [
                                MinusShape
                            ]
                        }
                    ]
                }
            })
        }
    ]
}