package com.kaimuellercode.todos_backend;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.jayway.jsonpath.JsonPath;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.MvcResult;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;

import static org.springframework.test.web.servlet.result.MockMvcResultHandlers.print;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;


@AutoConfigureMockMvc
@SpringBootTest
class ToDosBackendApplicationTests {
    @Autowired
    MockMvc mockMvc;


    @Autowired
    private ObjectMapper objectMapper;

    @Test
    void testTodoOperations() throws Exception {
        // Create a new todo item
        String newItemName = "Buy groceries";
        createTodoItem(newItemName);

        // Retrieve all todo items and check if the new item is present

        MvcResult result = mockMvc.perform(MockMvcRequestBuilders
                        .get("/todo/all")
                        .accept(MediaType.APPLICATION_JSON)
                )
                .andDo(print())
                .andExpect(status().isOk())
                .andExpect(jsonPath("$[0].name").value(newItemName))
                .andReturn();


        String response = result.getResponse().getContentAsString();
        Integer id = JsonPath.parse(response).read("$[0].id");

        // Delete the todo item
        deleteTodoItem(id);

        // Retrieve all todo items and check if the item is deleted
        mockMvc.perform(MockMvcRequestBuilders.get("/todo/all"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$").isEmpty());
    }

    private void createTodoItem(String itemName) throws Exception {
        mockMvc.perform(MockMvcRequestBuilders.post("/todo/new")
                        .param("item", itemName))
                .andDo(print())
                .andExpect(status().isOk());
    }

    private void deleteTodoItem(Integer id) throws Exception {
        mockMvc.perform(MockMvcRequestBuilders.post("/todo/delete")
                        .param("id", String.valueOf(id)))
                .andExpect(status().isOk());
    }
}

