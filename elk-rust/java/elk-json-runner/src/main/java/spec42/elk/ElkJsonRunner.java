package spec42.elk;

import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.stream.Collectors;

import org.eclipse.elk.core.RecursiveGraphLayoutEngine;
import org.eclipse.elk.core.util.BasicProgressMonitor;
import org.eclipse.elk.graph.ElkNode;
import org.eclipse.elk.graph.json.ElkGraphJson;

/**
 * Headless runner:
 *  - reads ELK Graph JSON from a file path arg or stdin
 *  - imports to ElkGraph
 *  - runs RecursiveGraphLayoutEngine
 *  - prints laid-out ELK Graph JSON to stdout
 */
public final class ElkJsonRunner {
  private ElkJsonRunner() {}

  private static String readAll(java.io.InputStream in) throws Exception {
    try (BufferedReader br = new BufferedReader(new InputStreamReader(in, StandardCharsets.UTF_8))) {
      return br.lines().collect(Collectors.joining("\n"));
    }
  }

  public static void main(String[] args) throws Exception {
    String json;
    if (args.length >= 1 && !args[0].isBlank()) {
      try (FileInputStream fis = new FileInputStream(args[0])) {
        json = readAll(fis);
      }
    } else {
      json = readAll(System.in);
    }

    // Import JSON -> ELK graph
    ElkNode root = ElkGraphJson.forGraph(json).toElk();

    // Run layout
    RecursiveGraphLayoutEngine engine = new RecursiveGraphLayoutEngine();
    engine.layout(root, new BasicProgressMonitor());

    // Export ELK graph -> JSON
    String out = ElkGraphJson.forGraph(root)
        .omitLayout(false)
        .omitZeroDimension(false)
        .omitZeroPositions(false)
        .prettyPrint(true)
        .shortLayoutOptionKeys(false)
        .toJson();
    System.out.print(out);
  }
}

