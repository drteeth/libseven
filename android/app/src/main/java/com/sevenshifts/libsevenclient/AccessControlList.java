package com.sevenshifts.libsevenclient;

import android.content.res.AssetManager;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;

public class AccessControlList {
    static {
        System.loadLibrary("seven");
    }

    private final String modelText;
    private final String[] policyLines;

    public static AccessControlList createFromFiles(AssetManager assets, String modelFilename, String policyFilename) {
        String model = readModelFile(assets, modelFilename);
        String[] policy = readPolicyFile(assets, policyFilename);
        return new AccessControlList(model, policy);
    }

    public AccessControlList(String model, String[] policy) {
        this.modelText = model;
        this.policyLines = policy;
    }

    public boolean enforce(String[] request) {
        return enforce(modelText, policyLines, request);
    }

    private native boolean enforce(String model, String[] policyLines, String[] request);

    private static String[] readPolicyFile(AssetManager assets, String filename) {
        try (InputStream what = assets.open(filename)) {
            InputStreamReader inputStreamReader;
            inputStreamReader = new InputStreamReader(what);
            BufferedReader reader = new BufferedReader(inputStreamReader);

            List<String> lines = new ArrayList<>();
            for (String line; (line = reader.readLine()) != null; ) {
                lines.add(line);
            }

            String[] arrayOfLines = new String[lines.size()];
            return lines.toArray(arrayOfLines);
        } catch (IOException e) {
            throw new RuntimeException("Unable to read policy file: ".concat(filename), e);
        }
    }

    private static String readModelFile(AssetManager assets, String filename) {
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
}