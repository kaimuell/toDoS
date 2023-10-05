package com.kaimuellercode.todos_backend.todos;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/todo")
public class ToDoController {

    @Autowired
    ToDoRepository toDoRepository;

    @PostMapping("/new")
    public @ResponseBody String newItem(@RequestParam String item){
        ToDoItem toDoItem = new ToDoItem();
        toDoItem.setName(item);
        toDoRepository.save(toDoItem);
        return "ToDoItem - Saved";
    }

    @GetMapping("/all")
    public @ResponseBody Iterable<ToDoItem> getAllItems(){
        return toDoRepository.findAll();
    }

    @DeleteMapping("/delete")
    public @ResponseBody String deleteById(@RequestParam Long id){
        toDoRepository.deleteById(id);
        return "deleted";
    }




}
