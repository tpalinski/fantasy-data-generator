import data.Armors;
import data.Others;
import data.Suffixes;
import data.Weapons;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class Generator {
    public Generator() {}

    public void saveItems(List<String> items, List<String> prefixes) throws IOException {
        BufferedWriter writer = new BufferedWriter(new FileWriter("../../data/artefakty.csv", true));
        for (String item : items) {
            for (String prefix : prefixes) {
                String text;
                if (item.endsWith("a")) {
                    text = prefix + "a " + item;
                } else if ((item.endsWith("e") || item.endsWith("y")) && prefix.endsWith("k")) {
                    text = prefix + "ie " + item;
                } else if (item.endsWith("e") || item.endsWith("i") || item.endsWith("y")) {
                    text = prefix + "e " + item;
                } else if (prefix.endsWith("k") || prefix.endsWith("g")) {
                    text = prefix + "i " + item;
                } else {
                    text = prefix + "y " + item;
                }
                for(String suffix : Suffixes.getSuffixes())
                {
                    String finalText = text + " " + suffix + ",\n";
                    writer.append(finalText);
                }
            }
        }
        writer.close();
    }
}
