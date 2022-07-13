package com.sevenshifts.libsevenclient;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.TextView;

import java.util.HashMap;
import java.util.Map;

public class MainActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        TextView sevenShiftsView = findViewById(R.id.seven_shifts);
        sevenShiftsView.setText(evaluate7Shifts());
    }

    @NonNull
    private String evaluate7Shifts() {
        String modelFile = "7shifts.conf";
        String policyFile = "7shifts.csv";
        AccessControlList acl = AccessControlList.createFromFiles(getAssets(), modelFile, policyFile);

        User user = new User("55", "org:11");
        Map<String, String> subject = new HashMap<>();
        subject.put("id", user.id);
        subject.put("org", user.companyId);

        try {
            return acl.enforce(subject, "company:11", "user", "read");
        } catch(Exception e) {
            return "7shifts" + " failed: " + e.getLocalizedMessage();
        }
    }
}