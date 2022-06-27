package com.sevenshifts.libsevenclient;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        TextView outputView = findViewById(R.id.output);

//        RustGreeting greeting = new RustGreeting();
//        String hello = greeting.sayHello("rawr");
//        outputView.setText(hello);

        outputView.setText(evaluateAcl());
    }

    @NonNull
    private String evaluateAcl() {
        String modelFile = "acl.conf";
        String policyFile = "acl.csv";
        AccessControlList acl = AccessControlList.createFromFiles(getAssets(), modelFile, policyFile);

        String[] request = new String[]{"alice", "data1", "read"};
        return evaluate("ACL", acl, request);
    }

    @NonNull
    private String evaluate(String name, AccessControlList acl, String[] stuff) {
        try {
            if (acl.enforce(stuff)) {
                return String.format("%s is OK", name);
            } else {
                return String.format("%s is REFUSED", name);
            }
        } catch(Exception e) {
            return name + " failed: " + e.getLocalizedMessage();
        }
    }
}