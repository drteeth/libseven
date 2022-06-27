package com.sevenshifts.libsevenclient;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        TextView outputView = findViewById(R.id.output);

        RustGreeting greeting = new RustGreeting();
        String hello = greeting.sayHello("rawr");
        outputView.setText(hello);
    }
}