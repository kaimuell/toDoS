package com.kaimuellercode.todos_backend.todos;

import org.springframework.data.repository.CrudRepository;

public interface ToDoRepository extends CrudRepository<ToDoItem, Long> {

}
