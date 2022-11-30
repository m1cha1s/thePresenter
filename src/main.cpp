#include <stdio.h>

#include <raylib.h>

#define RAYGUI_IMPLEMENTATION
#include <raygui.h>

#define P_WIDTH 800
#define P_HEIGHT 800
#define P_FPS 60

int main()
{
  InitWindow(P_WIDTH, P_HEIGHT, "thePresenter");

  SetTargetFPS(P_FPS);

  while (!WindowShouldClose())
  {
    BeginDrawing();
    {
      ClearBackground(RAYWHITE);
      DrawText("Hello world!", 400, 400, 21, BLACK);

      if (GuiButton((Rectangle){200, 200, 80, 30}, "Test Button"))
      {
        printf("[Test button] pressed\n");
      }
    }
    EndDrawing();
  }

  CloseWindow();
  return 0;
}