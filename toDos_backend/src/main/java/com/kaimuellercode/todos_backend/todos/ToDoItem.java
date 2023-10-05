package com.kaimuellercode.todos_backend.todos;

import jakarta.persistence.*;

@Entity
@Table(name="todo_items")
public class ToDoItem {

    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private Long id;

    private String name;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Long getId() {
        return id;
    }
}
