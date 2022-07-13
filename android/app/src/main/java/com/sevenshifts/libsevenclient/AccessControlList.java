package com.sevenshifts.libsevenclient;

import android.content.res.AssetManager;

import com.google.gson.Gson;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.Map;

public class AccessControlList {
    static {
        System.loadLibrary("seven");
    }

    private final String modelText;
    private final String policy;

    public AccessControlList(String model, String policy) {
        this.modelText = model;
        this.policy = policy;
    }

    public static AccessControlList createFromFiles(AssetManager assets, String modelFilename, String policyFilename) {
        String model = readFile(assets, modelFilename);
        String policy = readFile(assets, policyFilename);
        return new AccessControlList(model, policy);
    }

    private static String readFile(AssetManager assets, String filename) {
        try (InputStream what = assets.open(filename)) {
            InputStreamReader inputStreamReader = new InputStreamReader(what);
            BufferedReader reader = new BufferedReader(inputStreamReader);
            StringBuilder modelText = new StringBuilder();

            for (String line; (line = reader.readLine()) != null; ) {
                modelText.append(line).append('\n');
            }

            return modelText.toString();
        } catch (IOException e) {
            throw new RuntimeException("Unable to read model file: ".concat(filename), e);
        }
    }

    public String enforce(Map<String, String> subject, String companyId, String resource, String action) {
        Gson gson = new Gson();
        String subjectJSON = gson.toJson(subject);
        return enforce(this.modelText, this.policy, subjectJSON, companyId, resource, action);
    }

    private native String enforce(String model, String policy, String subjectJSON, String organizationalUnit, String resource, String action);
}